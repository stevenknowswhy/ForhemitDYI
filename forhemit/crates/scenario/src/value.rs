//! Typed values (Scenario Engine Data Model doc §8): a scenario
//! assumption is never a bare string — every value carries its shape, and
//! a value's shape must match its declared `value_type`.

use serde::{Deserialize, Serialize};

/// A decimal number held as a canonical string.
///
/// Money, ownership percentages, and modeled proceeds must not travel
/// through binary floats — a rounding drift at the tenth of a percent is
/// a silent data corruption in an ownership-transition tool. The value is
/// stored exactly as the source stated it and validated as a plain
/// decimal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixedDecimal(String);

impl FixedDecimal {
    /// Parses and canonicalizes a decimal string: optional `-`, digits,
    /// and at most one fraction. Trailing zeros are normalized away so
    /// equal values compare equal.
    ///
    /// # Errors
    /// [`ScenarioError::InvalidValue`] when the text is not a decimal
    /// number.
    pub fn parse(text: &str) -> Result<Self, crate::ScenarioError> {
        let trimmed = text.trim();
        let invalid = || crate::ScenarioError::InvalidValue {
            reason: "expected a decimal number (e.g. \"2400000\" or \"37.5\")",
        };
        if trimmed.is_empty() {
            return Err(invalid());
        }
        let (sign, rest) = match trimmed.strip_prefix('-') {
            Some(rest) => (-1i8, rest),
            None => (1, trimmed),
        };
        let (int_part, frac_part) = match rest.split_once('.') {
            Some((int_part, frac_part)) => (int_part, Some(frac_part)),
            None => (rest, None),
        };
        if int_part.is_empty() || !int_part.bytes().all(|b| b.is_ascii_digit()) {
            return Err(invalid());
        }
        if let Some(frac_part) = frac_part {
            if !frac_part.bytes().all(|b| b.is_ascii_digit()) {
                return Err(invalid());
            }
        }
        let is_zero = int_part.bytes().all(|b| b == b'0')
            && frac_part.is_none_or(|frac| frac.bytes().all(|b| b == b'0'));
        let canonical = if is_zero {
            "0".to_owned()
        } else {
            let mut int_trimmed = int_part.trim_start_matches('0');
            if int_trimmed.is_empty() {
                int_trimmed = "0";
            }
            match (frac_part, sign) {
                (Some(frac), _) => {
                    let frac = frac.trim_end_matches('0');
                    if frac.is_empty() {
                        int_trimmed.to_owned()
                    } else {
                        format!("{int_trimmed}.{frac}")
                    }
                }
                (None, -1) => format!("-{int_trimmed}"),
                (None, _) => int_trimmed.to_owned(),
            }
        };
        Ok(Self(canonical))
    }

    /// The canonical decimal text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for FixedDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Sign, integer digits, and fraction digits of a canonical decimal.
fn decompose_decimal(text: &str) -> (bool, &str, &str) {
    let (negative, rest) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let (int_part, frac_part) = rest.split_once('.').unwrap_or((rest, ""));
    (negative, int_part, frac_part)
}

impl PartialOrd for FixedDecimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FixedDecimal {
    /// Exact numeric ordering over the canonical text. Lexicographic
    /// string comparison would misorder different integer lengths
    /// (`"10" < "9"` as text), so integer parts compare by length first;
    /// fraction parts compare digit by digit, which is correct because
    /// both are the same number's fractional expansion.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        let (neg_a, int_a, frac_a) = decompose_decimal(&self.0);
        let (neg_b, int_b, frac_b) = decompose_decimal(&other.0);
        let magnitude = int_a
            .len()
            .cmp(&int_b.len())
            .then_with(|| int_a.cmp(int_b))
            .then_with(|| frac_a.cmp(frac_b));
        match (neg_a, neg_b) {
            (false, false) => magnitude,
            (true, true) => magnitude.reverse(),
            // Canonical zero never carries a sign, so a mixed sign pair
            // is always strictly ordered.
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
        }
    }
}

/// An ISO 4217-style currency code — exactly three uppercase ASCII letters.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyCode(String);

impl CurrencyCode {
    /// Validates a currency code ("USD", "EUR", …).
    ///
    /// # Errors
    /// [`ScenarioError::InvalidValue`] when the code is not three
    /// uppercase ASCII letters.
    pub fn parse(code: &str) -> Result<Self, crate::ScenarioError> {
        let invalid = || crate::ScenarioError::InvalidValue {
            reason: "expected a three-letter uppercase currency code (e.g. \"USD\")",
        };
        let bytes = code.as_bytes();
        if bytes.len() == 3 && bytes.iter().all(u8::is_ascii_uppercase) {
            Ok(Self(code.to_owned()))
        } else {
            Err(invalid())
        }
    }

    /// The code text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// The typed value vocabulary (schema doc §8). The engine stores the
/// variant and derives the `value_type` column from it, so the declared
/// shape and the actual value cannot disagree.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "value_type", content = "value", rename_all = "snake_case", deny_unknown_fields)]
pub enum TypedValue {
    /// A plain decimal number (e.g. an employee count).
    Number(FixedDecimal),
    /// A money amount with its currency.
    Currency {
        /// The amount, exact.
        amount: FixedDecimal,
        /// The ISO currency code.
        currency: CurrencyCode,
    },
    /// A percentage, 0–100.
    Percentage(FixedDecimal),
    /// A calendar date.
    Date(time::Date),
    /// A date range, `from` ≤ `to`.
    DateRange {
        /// First day of the range.
        from: time::Date,
        /// Last day of the range.
        to: time::Date,
    },
    /// A duration in whole months (ownership transitions are negotiated
    /// in months, not seconds).
    DurationMonths(u32),
    /// A yes/no condition.
    Boolean(bool),
    /// Free text — never empty; use the absence of a value instead.
    Text(String),
    /// A key from a controlled vocabulary, as the journey defines it.
    Enum(String),
    /// A numeric range; `max` is None for an open-ended bound.
    Range {
        /// Lower bound.
        min: FixedDecimal,
        /// Upper bound; `None` = open-ended.
        max: Option<FixedDecimal>,
    },
    /// A reference to an object stored elsewhere (schema doc §6:
    /// "reference authoritative versions rather than duplicate").
    Reference(String),
}

impl TypedValue {
    /// Validates the value's internal consistency (range bounds, empty
    /// text, percentage bounds).
    ///
    /// # Errors
    /// [`ScenarioError::InvalidValue`] when the value contradicts itself.
    pub fn validate(&self) -> Result<(), crate::ScenarioError> {
        match self {
            Self::Percentage(pct) => {
                let value = pct
                    .as_str()
                    .parse::<f64>()
                    .map_err(|_| crate::ScenarioError::InvalidValue {
                        reason: "a percentage must be a number",
                    })?;
                if !(0.0..=100.0).contains(&value) {
                    return Err(crate::ScenarioError::InvalidValue {
                        reason: "a percentage must be between 0 and 100",
                    });
                }
                Ok(())
            }
            Self::DateRange { from, to } if from > to => Err(crate::ScenarioError::InvalidValue {
                reason: "date range 'from' must not be after 'to'",
            }),
            Self::Range {
                min,
                max: Some(max),
            } if min > max => Err(crate::ScenarioError::InvalidValue {
                reason: "range 'min' must not be greater than 'max'",
            }),
            Self::Text(text) | Self::Enum(text) | Self::Reference(text)
                if text.trim().is_empty() =>
            {
                Err(crate::ScenarioError::InvalidValue {
                    reason: "text values must not be empty; use the absence of a value",
                })
            }
            _ => Ok(()),
        }
    }

    /// The canonical `value_type` column value for this variant (schema
    /// doc §8 / SCHEMA.md §6.5).
    pub fn value_type(&self) -> &'static str {
        match self {
            Self::Number(_) => "number",
            Self::Currency { .. } => "currency",
            Self::Percentage(_) => "percentage",
            Self::Date(_) => "date",
            Self::DateRange { .. } => "date_range",
            Self::DurationMonths(_) => "duration",
            Self::Boolean(_) => "boolean",
            Self::Text(_) => "text",
            Self::Enum(_) => "enum",
            Self::Range { .. } => "range",
            Self::Reference(_) => "reference",
        }
    }
}
