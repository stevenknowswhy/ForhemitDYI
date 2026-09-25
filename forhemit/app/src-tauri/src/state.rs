//! The composition root: wires the engines to the concrete audit store,
//! clock, and journey store for one workspace, and holds the app's domain
//! state.
//!
//! The shell owns no domain logic — every mutation goes through an engine
//! API, and every engine emission lands in the append-only audit store
//! through [`TeeSink`], which mirrors accepted events into the session log
//! the UI's audit panel displays. The audit stream is the durable record;
//! the shell additionally persists the destination aggregate and the
//! active journey pointer as JSON beside the SQLite stores so a walk and
//! its North Star survive an app restart.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use forhemit_audit::AuditStore;
use forhemit_contracts::{ActorRecord, AuditEventDraft, JourneyInstanceId, WorkspaceId};
use forhemit_destination::Destination;
use forhemit_enginekit::{AuditError, AuditSink, Clock, SystemClock};
use forhemit_journey::{JourneyDefinition, SqliteJourneyStore};
use forhemit_reality::RealityEngine;
use serde::{Deserialize, Serialize};

/// The shell's workspace identity. v1 is single-workspace, single-device.
pub const WORKSPACE_ID: &str = "local-workspace";

/// The owner actor every UI-initiated command carries (Audit doc §7).
#[must_use]
pub fn owner_actor() -> ActorRecord {
    forhemit_destination::engine::owner_actor("local-owner")
}

/// One accepted audit event, as the UI's audit panel shows it.
#[derive(Clone, Debug, Serialize)]
pub struct AuditLogLine {
    /// When the event was accepted (RFC 3339).
    pub at: String,
    /// The event type, engine-namespaced (e.g. `JourneyAnswerRecorded`).
    pub event_type: String,
    /// The engine that produced the event.
    pub engine: String,
    /// The object the event is about.
    pub object: String,
}

/// Display cap of the session log. The audit store keeps every event; this
/// only bounds the dev panel's memory.
const SESSION_LOG_CAP: usize = 500;

/// The shell-side audit sink: appends to the real store, then mirrors the
/// accepted event into the session log. Engines still cannot read, edit,
/// or delete history — the log is a display projection of accepted
/// events, never a second record.
pub struct TeeSink {
    store: AuditStore,
    log: Arc<Mutex<Vec<AuditLogLine>>>,
    clock: Arc<dyn Clock>,
}

impl AuditSink<AuditEventDraft> for TeeSink {
    fn emit(&self, event: AuditEventDraft) -> Result<(), AuditError> {
        // Audit first: the line is displayed only once the event is
        // actually in the store.
        self.store.emit(event.clone())?;
        let line = AuditLogLine {
            at: rfc3339_now(&self.clock),
            event_type: format!("{:?}", event.event_type),
            engine: format!("{:?}", event.source_engine),
            object: event.source_object.as_str().to_owned(),
        };
        let mut log = self
            .log
            .lock()
            .map_err(|_| AuditError("session log lock poisoned".to_owned()))?;
        log.push(line);
        let excess = log.len().saturating_sub(SESSION_LOG_CAP);
        if excess > 0 {
            log.drain(..excess);
        }
        Ok(())
    }
}

/// The shell's domain state: the destination aggregate, the active walk,
/// and the session audit log.
pub struct AppEngines {
    /// The workspace every engine writes under.
    pub workspace_id: WorkspaceId,
    /// The owner actor for UI-initiated commands.
    pub actor: ActorRecord,
    /// The append-only audit store.
    pub audit: Arc<AuditStore>,
    /// The system clock engines use for timestamps.
    pub clock: Arc<dyn Clock>,
    /// The journey instance store (SQLite).
    pub journey_store: Arc<SqliteJourneyStore>,
    /// The tee sink every engine emits through — also the destination and
    /// journey engines' sink.
    pub tee: Arc<TeeSink>,
    /// The Business Reality engine over the shared sink.
    pub reality: RealityEngine<TeeSink>,
    /// The pinned EOJ v0.2 definition the app walks.
    pub definition: JourneyDefinition,
    data_dir: PathBuf,
    log: Arc<Mutex<Vec<AuditLogLine>>>,
    session: Mutex<Session>,
}

struct Session {
    destination: Option<Destination>,
    active_journey: Option<JourneyInstanceId>,
}

/// Formats the clock's current time as RFC 3339. Formatting a valid
/// `OffsetDateTime` against Rfc3339 cannot fail; a failure renders empty
/// rather than fabricating a timestamp.
fn rfc3339_now(clock: &Arc<dyn Clock>) -> String {
    use time::format_description::well_known::Rfc3339;
    clock.now().format(&Rfc3339).unwrap_or_default()
}

/// Serializes `value` to a pretty JSON file.
fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let json =
        serde_json::to_string_pretty(value).map_err(|error| format!("serialize: {error}"))?;
    std::fs::write(path, json).map_err(|error| format!("write {}: {error}", path.display()))
}

/// Reads a JSON file; `None` when it does not exist yet (fresh workspace).
fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<Option<T>, String> {
    if !path.exists() {
        return Ok(None);
    }
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("read {}: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map(Some)
        .map_err(|error| format!("parse {}: {error}", path.display()))
}

impl AppEngines {
    /// Opens the workspace: the audit store and journey store under
    /// `data_dir`, the destination aggregate from its persisted file, and
    /// the pinned EOJ v0.2 definition.
    ///
    /// # Errors
    ///
    /// A store that cannot open, a persisted document that no longer
    /// parses (corruption or a downgrade reading newer data), or a
    /// journey data file that fails validation.
    pub fn open(data_dir: &Path) -> Result<Self, String> {
        std::fs::create_dir_all(data_dir)
            .map_err(|error| format!("cannot create data dir: {error}"))?;
        let clock: Arc<dyn Clock> = Arc::new(SystemClock);
        let audit = AuditStore::open(data_dir.join("audit.sqlite3"), clock.clone())
            .map_err(|error| error.to_string())?;
        let journey_store = SqliteJourneyStore::open(&data_dir.join("journey.sqlite3"))
            .map_err(|error| error.to_string())?;

        let log = Arc::new(Mutex::new(Vec::<AuditLogLine>::new()));
        let tee = Arc::new(TeeSink {
            store: audit.clone(),
            log: log.clone(),
            clock: clock.clone(),
        });
        let workspace_id = WorkspaceId::new(WORKSPACE_ID).map_err(|error| error.to_string())?;
        let reality = RealityEngine::new(tee.clone(), clock.clone(), workspace_id.clone());
        let definition =
            forhemit_journey::load_employee_ownership_v0_2().map_err(|error| error.to_string())?;

        let destination = read_json::<Destination>(&data_dir.join("destination.json"))?;
        let active_journey = read_json::<ActiveJourney>(&data_dir.join("active_journey.json"))?
            .map(|pointer| JourneyInstanceId::new(pointer.instance_id))
            .transpose()
            .map_err(|error| error.to_string())?;

        Ok(Self {
            workspace_id,
            actor: owner_actor(),
            audit: Arc::new(audit),
            clock,
            journey_store: Arc::new(journey_store),
            tee,
            reality,
            definition,
            data_dir: data_dir.to_path_buf(),
            log,
            session: Mutex::new(Session {
                destination,
                active_journey,
            }),
        })
    }

    /// The session log, oldest first — the audit panel's display source.
    ///
    /// # Errors
    ///
    /// The log lock is poisoned (a panic while it was held).
    pub fn audit_log(&self) -> Result<Vec<AuditLogLine>, String> {
        Ok(self.log.lock().map_err(|_| "log lock poisoned")?.clone())
    }

    /// Runs `f` with the current destination, if one exists.
    pub fn with_destination<R>(
        &self,
        f: impl FnOnce(Option<&Destination>) -> R,
    ) -> Result<R, String> {
        let session = self.session.lock().map_err(|_| "session lock poisoned")?;
        Ok(f(session.destination.as_ref()))
    }

    /// Replaces the destination aggregate (create/clear) and persists it.
    ///
    /// # Errors
    ///
    /// The session lock is poisoned or the file cannot be written.
    pub fn set_destination(&self, destination: Option<Destination>) -> Result<(), String> {
        let mut session = self.session.lock().map_err(|_| "session lock poisoned")?;
        session.destination = destination;
        match &session.destination {
            Some(destination) => write_json(&self.data_dir.join("destination.json"), destination),
            // No destination means the aggregate was cleared; the persisted
            // file (if any) must not survive as a stale copy.
            None => {
                let path = self.data_dir.join("destination.json");
                if path.exists() {
                    std::fs::remove_file(&path)
                        .map_err(|error| format!("remove destination.json: {error}"))?;
                }
                Ok(())
            }
        }
    }

    /// Runs `f` with the current destination mutably, persisting the
    /// aggregate afterwards.
    pub fn with_destination_mut<R>(
        &self,
        f: impl FnOnce(Option<&mut Destination>) -> R,
    ) -> Result<R, String> {
        let mut session = self.session.lock().map_err(|_| "session lock poisoned")?;
        let result = f(session.destination.as_mut());
        match &session.destination {
            Some(destination) => write_json(&self.data_dir.join("destination.json"), destination)?,
            // No destination means the aggregate was cleared; the persisted
            // file (if any) must not survive as a stale copy.
            None => {
                let path = self.data_dir.join("destination.json");
                if path.exists() {
                    std::fs::remove_file(&path)
                        .map_err(|error| format!("remove destination.json: {error}"))?;
                }
            }
        }
        Ok(result)
    }

    /// The active journey instance id, if a walk was started.
    ///
    /// # Errors
    ///
    /// The session lock is poisoned.
    pub fn active_journey(&self) -> Result<Option<JourneyInstanceId>, String> {
        Ok(self
            .session
            .lock()
            .map_err(|_| "session lock poisoned")?
            .active_journey
            .clone())
    }

    /// Points the workspace at `instance_id` and persists the pointer.
    ///
    /// # Errors
    ///
    /// The session lock is poisoned or the pointer file cannot be written.
    pub fn set_active_journey(&self, instance_id: &JourneyInstanceId) -> Result<(), String> {
        let mut session = self.session.lock().map_err(|_| "session lock poisoned")?;
        session.active_journey = Some(instance_id.clone());
        write_json(
            &self.data_dir.join("active_journey.json"),
            &ActiveJourney {
                instance_id: instance_id.as_str().to_owned(),
            },
        )
    }
}

/// The journey pointer persisted beside the stores — the instance the app
/// resumes on restart.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct ActiveJourney {
    instance_id: String,
}
