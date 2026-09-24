//! Journey instance persistence — the resumable snapshot of a walk.
//!
//! The spec requires state that "persists across app restarts"; the
//! durable form is a [`JourneyInstanceDocument`] saved through
//! [`JourneyStore`]. The SQLite store (WAL) is the production
//! implementation; the in-memory store backs tests and the app shell's
//! pre-vault wiring. Stored documents are versioned — an unknown
//! instance version fails loudly ([`JourneyError::StoreState`]) instead
//! of being reinterpreted.

use std::collections::BTreeMap;
use std::sync::Mutex;

use rusqlite::Connection;

use crate::error::JourneyError;
use crate::instance::JourneyInstanceDocument;

/// Persistence boundary of the journey runtime: engines save and load
/// versioned walk documents, never raw rows.
pub trait JourneyStore: Send + Sync {
    /// Persists the current snapshot of one walk — the document fully
    /// replaces any earlier snapshot for the same instance.
    ///
    /// # Errors
    ///
    /// Storage failure or a document the store cannot serialize.
    fn save(&self, document: &JourneyInstanceDocument) -> Result<(), JourneyError>;

    /// The stored snapshot for one walk, if any.
    ///
    /// # Errors
    ///
    /// Storage failure or a stored document that no longer parses.
    fn load(
        &self,
        instance_id: &forhemit_contracts::JourneyInstanceId,
    ) -> Result<Option<JourneyInstanceDocument>, JourneyError>;

    /// Every stored walk, oldest update first — the "continue where you
    /// left off" list.
    ///
    /// # Errors
    ///
    /// Storage failure or a stored document that no longer parses.
    fn list(&self) -> Result<Vec<JourneyInstanceDocument>, JourneyError>;
}

/// In-memory journey store — tests and pre-persistence wiring.
#[derive(Default)]
pub struct MemoryJourneyStore {
    instances: Mutex<BTreeMap<String, JourneyInstanceDocument>>,
}

impl MemoryJourneyStore {
    /// An empty store.
    pub fn new() -> Self {
        Self::default()
    }
}

impl JourneyStore for MemoryJourneyStore {
    fn save(&self, document: &JourneyInstanceDocument) -> Result<(), JourneyError> {
        let mut instances = self
            .instances
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        instances.insert(
            document.state().instance_id.as_str().to_owned(),
            document.clone(),
        );
        Ok(())
    }

    fn load(
        &self,
        instance_id: &forhemit_contracts::JourneyInstanceId,
    ) -> Result<Option<JourneyInstanceDocument>, JourneyError> {
        let instances = self
            .instances
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        Ok(instances.get(instance_id.as_str()).cloned())
    }

    fn list(&self) -> Result<Vec<JourneyInstanceDocument>, JourneyError> {
        let instances = self
            .instances
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut documents: Vec<JourneyInstanceDocument> = instances.values().cloned().collect();
        documents.sort_by_key(|left| left.state().updated_at);
        Ok(documents)
    }
}

/// SQLite journey store — WAL journal, one row per walk, the versioned
/// document serialized as JSON.
pub struct SqliteJourneyStore {
    connection: Mutex<Connection>,
}

impl SqliteJourneyStore {
    /// Opens (creating if needed) a store at `path`, with WAL journalling.
    ///
    /// # Errors
    ///
    /// [`JourneyError::Store`] when the file cannot be opened or the
    /// schema cannot be ensured.
    pub fn open(path: &std::path::Path) -> Result<Self, JourneyError> {
        let connection = Connection::open(path)?;
        connection.pragma_update(None, "journal_mode", "WAL")?;
        Self::with_connection(connection)
    }

    /// Opens a store over an in-memory database — self-contained tests.
    ///
    /// # Errors
    ///
    /// [`JourneyError::Store`] when the schema cannot be ensured.
    pub fn open_in_memory() -> Result<Self, JourneyError> {
        Self::with_connection(Connection::open_in_memory()?)
    }

    fn with_connection(connection: Connection) -> Result<Self, JourneyError> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS journey_instances (
                instance_id TEXT PRIMARY KEY,
                journey_id TEXT NOT NULL,
                journey_version TEXT NOT NULL,
                status TEXT NOT NULL,
                updated_at INTEGER NOT NULL,
                document TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }
}

impl JourneyStore for SqliteJourneyStore {
    fn save(&self, document: &JourneyInstanceDocument) -> Result<(), JourneyError> {
        let state = document.state();
        let json = serde_json::to_string(document).map_err(JourneyError::Serialization)?;
        // Integer nanos, the audit store's timestamp shape — no
        // string-format round trips.
        let updated_at = i64::try_from(state.updated_at.unix_timestamp_nanos()).map_err(|_| {
            JourneyError::Store(rusqlite::Error::ToSqlConversionFailure(
                "journey timestamp out of storable range".into(),
            ))
        })?;
        let connection = self
            .connection
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        connection.execute(
            "INSERT INTO journey_instances
                (instance_id, journey_id, journey_version, status, updated_at, document)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(instance_id) DO UPDATE SET
                journey_id = excluded.journey_id,
                journey_version = excluded.journey_version,
                status = excluded.status,
                updated_at = excluded.updated_at,
                document = excluded.document",
            rusqlite::params![
                state.instance_id.as_str(),
                state.journey_id.as_str(),
                state.journey_version,
                match state.status {
                    crate::instance::InstanceStatus::InProgress => "in_progress",
                    crate::instance::InstanceStatus::Completed => "completed",
                },
                updated_at,
                json,
            ],
        )?;
        Ok(())
    }

    fn load(
        &self,
        instance_id: &forhemit_contracts::JourneyInstanceId,
    ) -> Result<Option<JourneyInstanceDocument>, JourneyError> {
        let connection = self
            .connection
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut statement =
            connection.prepare("SELECT document FROM journey_instances WHERE instance_id = ?1")?;
        let mut rows = statement.query([instance_id.as_str()])?;
        match rows.next()? {
            Some(row) => {
                let json: String = row.get(0)?;
                Ok(Some(
                    serde_json::from_str(&json).map_err(JourneyError::StoreState)?,
                ))
            }
            None => Ok(None),
        }
    }

    fn list(&self) -> Result<Vec<JourneyInstanceDocument>, JourneyError> {
        let connection = self
            .connection
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut statement =
            connection.prepare("SELECT document FROM journey_instances ORDER BY updated_at ASC")?;
        let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
        let mut documents = Vec::new();
        for row in rows {
            let json: String = row?;
            documents.push(serde_json::from_str(&json).map_err(JourneyError::StoreState)?);
        }
        Ok(documents)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use forhemit_contracts::CorrelationId;

    use super::*;
    use crate::instance::JourneyInstance;

    fn document_for(instance_id: &str) -> JourneyInstanceDocument {
        let now = time::OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let definition = crate::load_employee_ownership_v0_2().unwrap();
        JourneyInstance::start(
            forhemit_contracts::JourneyInstanceId::new(instance_id).unwrap(),
            CorrelationId::new("cor_1").unwrap(),
            &definition,
            now,
        )
        .to_document()
    }

    #[test]
    fn memory_store_saves_loads_and_lists() {
        let store = MemoryJourneyStore::new();
        let document = document_for("inst_1");
        store.save(&document).unwrap();
        let loaded = store
            .load(&forhemit_contracts::JourneyInstanceId::new("inst_1").unwrap())
            .unwrap();
        assert_eq!(loaded, Some(document));
        assert_eq!(store.list().unwrap().len(), 1);
        assert!(store
            .load(&forhemit_contracts::JourneyInstanceId::new("inst_missing").unwrap())
            .unwrap()
            .is_none());
    }

    #[test]
    fn sqlite_store_round_trips_and_replaces() {
        let store = SqliteJourneyStore::open_in_memory().unwrap();
        let instance_id = forhemit_contracts::JourneyInstanceId::new("inst_1").unwrap();
        store.save(&document_for("inst_1")).unwrap();

        // A second save of the same instance fully replaces the snapshot.
        let mut state = store.load(&instance_id).unwrap().unwrap();
        state.state_mut().updated_at += time::Duration::seconds(5);
        store.save(&state).unwrap();

        let loaded = store.load(&instance_id).unwrap().unwrap();
        assert_eq!(loaded, state);
        assert_eq!(store.list().unwrap().len(), 1);
    }

    #[test]
    fn sqlite_store_persists_across_reopen() {
        // The restart test: one store writes, the process "restarts"
        // (the store is dropped and a fresh store opens the same file),
        // and the walk comes back exactly as it left.
        let path = std::env::temp_dir().join(format!(
            "forhemit-journey-store-{}-{}.sqlite",
            std::process::id(),
            ulid::Ulid::new()
        ));
        let instance_id = forhemit_contracts::JourneyInstanceId::new("inst_1").unwrap();
        {
            let store = SqliteJourneyStore::open(&path).unwrap();
            store.save(&document_for("inst_1")).unwrap();
        }
        {
            let store = SqliteJourneyStore::open(&path).unwrap();
            let loaded = store.load(&instance_id).unwrap().unwrap();
            assert_eq!(loaded.state().instance_id.as_str(), "inst_1");
            assert_eq!(loaded.state().journey_version, "0.2");
        }
        std::fs::remove_file(&path).ok();
    }
}
