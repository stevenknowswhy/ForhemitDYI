//! The answer vocabulary — "I'm not sure" is always a legitimate answer.
//!
//! Destination Builder design principle (Employee Ownership Journey -
//! Destination Builder v1.0 §1): *"'I'm not sure' is always a legitimate
//! answer. Never force false certainty."* The builder must also not
//! overquestion (Destination Engine doc §41): use **Unknown**, **Flexible**,
//! **Approximate** rather than manufacturing precision.

use serde::{Deserialize, Serialize};

/// One field's answer state.
///
/// [`Answer::NotSure`] is a first-class answer the owner explicitly chose —
/// never a gap to be filled or defaulted; [`Answer::Unanswered`] means the
/// field has not been addressed at all.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Answer<T> {
    /// The field has not been addressed yet.
    Unanswered,
    /// The owner explicitly answered "I'm not sure".
    NotSure,
    /// The owner's substantive answer.
    Answered(T),
}

impl<T> Answer<T> {
    /// The owner's substantive answer, if one was given.
    ///
    /// An explicit "I'm not sure" is deliberately *not* returned here: the
    /// completeness indicator treats it as not established (Destination
    /// Builder doc §25 — the indicator shows what "has been established").
    pub fn answered(&self) -> Option<&T> {
        match self {
            Answer::Answered(value) => Some(value),
            Answer::NotSure | Answer::Unanswered => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;

    #[test]
    fn not_sure_serializes_as_a_plain_string() {
        let answer: Answer<String> = Answer::NotSure;
        assert_eq!(serde_json::to_value(&answer).unwrap(), "not_sure");
    }

    #[test]
    fn answered_round_trips_with_strict_fields() {
        let answer: Answer<u32> = Answer::Answered(7);
        let json = serde_json::to_value(&answer).unwrap();
        let back: Answer<u32> = serde_json::from_value(json).unwrap();
        assert_eq!(back, answer);
    }

    #[test]
    fn unknown_keys_are_rejected() {
        let result = serde_json::from_str::<Answer<u32>>(r#"{"answered": 1, "extra": true}"#);
        assert!(result.is_err(), "unknown fields must fail deserialization");
    }

    #[test]
    fn answered_accessor_returns_only_substantive_answers() {
        let unsure: Answer<u32> = Answer::NotSure;
        assert_eq!(unsure.answered(), None);
        let answered: Answer<u32> = Answer::Answered(3);
        assert_eq!(answered.answered(), Some(&3));
    }
}
