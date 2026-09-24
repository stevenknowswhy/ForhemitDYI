//! Shared engine plumbing ("enginekit"): the audit emission interface and
//! injectable time.
//!
//! The `AuditEvent` contract itself lands with the audit store task; until
//! then [`AuditSink`] is generic over the event contract so engine crates
//! can already be written against the emission boundary.

/// Injection point for the current time, so time-dependent behavior is
/// testable deterministically.
pub trait Clock: Send + Sync {
    /// The current UTC time.
    fn now(&self) -> time::OffsetDateTime;
}

/// Production clock: the operating system's UTC time.
#[derive(Clone, Copy, Debug, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> time::OffsetDateTime {
        time::OffsetDateTime::now_utc()
    }
}

/// Error surface of the audit sink. Structured variants (tamper detection,
/// chain integrity) land with the audit store task.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditError(pub String);

impl std::fmt::Display for AuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for AuditError {}

/// The one interface engines use to reach the audit store — emission only:
/// engines can append events but never read, edit, or delete history
/// (corrections create additional events rather than rewriting history,
/// per Audit - Provenance Engine).
///
/// Generic over the event contract; the audit store task binds engines to
/// the concrete, versioned `AuditEvent` type.
pub trait AuditSink<E>: Send + Sync {
    /// Appends one event to the workspace's audit history.
    fn emit(&self, event: E) -> Result<(), AuditError>;
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;
    use std::sync::Mutex;

    /// Purely in-memory sink for engine tests.
    #[derive(Default)]
    pub struct MemorySink {
        events: Mutex<Vec<String>>,
    }

    impl MemorySink {
        /// Recorded event count so far.
        pub fn len(&self) -> usize {
            self.events.lock().unwrap().len()
        }
    }

    impl AuditSink<String> for MemorySink {
        fn emit(&self, event: String) -> Result<(), AuditError> {
            self.events.lock().unwrap().push(event);
            Ok(())
        }
    }

    #[test]
    fn system_clock_returns_utc() {
        let now = SystemClock.now();
        assert_eq!(now.offset(), time::UtcOffset::UTC);
    }

    #[test]
    fn memory_sink_records_events_in_order() {
        let sink = MemorySink::default();
        assert_eq!(sink.len(), 0);
        sink.emit("one".to_owned()).unwrap();
        sink.emit("two".to_owned()).unwrap();
        assert_eq!(sink.len(), 2);
        assert_eq!(sink.events.lock().unwrap()[0], "one");
    }
}
