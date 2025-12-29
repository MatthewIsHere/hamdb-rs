//! Utility types for parsing and validating amateur-radio callsigns before
//! submitting them to the HamDB API.

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Parsed representation of a callsign.
///
/// The `base` portion contains only uppercase ASCII letters and numbers. An
/// optional suffix (anything after `/`) is preserved if present.
pub struct ParsedCallsign {
    pub base: String,
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Error)]
pub enum CallsignParseError {
    #[error("no callsign was provided")]
    Empty,
    #[error("callsign `{input}` contained an invalid char '{ch}' at index {index}")]
    InvalidChar {
        input: String,
        ch: char,
        index: usize,
    },
    #[error("callsign `{input}` of length {len} was not within expected size")]
    InvalidLength { input: String, len: usize },
}

/// Parse a string into a [`ParsedCallsign`], validating general formatting.
///
/// The parser uppercases all ASCII letters, enforces a global length bound and
/// only allows alphanumeric characters. Any suffix (text after `/`) is also
/// uppercased and returned separately.
///
/// # Examples
/// ```
/// use hamdb::parsing::{parse_callsign, ParsedCallsign};
///
/// let parsed = parse_callsign("W1AW/AE").unwrap();
/// assert_eq!(parsed.base, "W1AW");
/// assert_eq!(parsed.suffix.as_deref(), Some("AE"));
/// ```
pub fn parse_callsign(input: &str) -> Result<ParsedCallsign, CallsignParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CallsignParseError::Empty);
    }

    let mut base = String::with_capacity(trimmed.len());
    let mut suffix = None;

    // Parse base up to first '/'
    for (i, c) in trimmed.chars().enumerate() {
        if c == '/' {
            // suffix is the remainder (if any), uppercased
            let end = &trimmed[i + 1..];
            let mut s = String::with_capacity(end.len());
            for (j, c) in end.chars().enumerate() {
                let up = c.to_ascii_uppercase();
                if !up.is_ascii_alphanumeric() {
                    // the +1 accounts for '/'
                    return Err(CallsignParseError::InvalidChar {
                        input: input.to_string(),
                        ch: c,
                        index: i + j + 1,
                    });
                }
                s.push(up);
            }
            suffix = match s.is_empty() {
                true => None,
                false => Some(s),
            };
            break;
        }

        let up = c.to_ascii_uppercase();
        if !up.is_ascii_alphanumeric() {
            return Err(CallsignParseError::InvalidChar {
                input: input.to_string(),
                ch: c,
                index: i,
            });
        }
        base.push(up);
    }

    if base.is_empty() {
        return Err(CallsignParseError::Empty);
    }

    // Reasonable global plausibility bounds
    const MIN_LEN: usize = 3;
    const MAX_LEN: usize = 10;

    if !(MIN_LEN..=MAX_LEN).contains(&base.len()) {
        return Err(CallsignParseError::InvalidLength {
            input: input.to_string(),
            len: base.len(),
        });
    }

    Ok(ParsedCallsign { base, suffix })
}
