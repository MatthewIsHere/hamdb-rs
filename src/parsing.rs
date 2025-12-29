use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCallsign {
    pub base: String,
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Error)]
pub enum CallsignParseError {
    #[error("no callsign was provided")]
    Empty,
    #[error("callsign `{input}` contained an invalid char '{ch}' at index {index}")]
    InvalidChar { input: String, ch: char, index: usize },
    #[error("callsign `{input}` of length {len} was not within expected size")]
    InvalidLength { input: String, len: usize },
}

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
            let end = &trimmed[i+1..];
            let mut s = String::with_capacity(end.len());
            for (j, c) in end.chars().enumerate() {
                let up = c.to_ascii_uppercase();
                if !up.is_ascii_alphanumeric() {
                    // the +1 accounts for '/'
                    return Err(CallsignParseError::InvalidChar { input: input.to_string(), ch: c, index: i+j+1 })
                }
                s.push(up);
            }
            suffix = match s.is_empty() {
                true => None,
                false => Some(s)
            };
            break;
        }

        let up = c.to_ascii_uppercase();
        if !up.is_ascii_alphanumeric() {
            return Err(CallsignParseError::InvalidChar { input: input.to_string(), ch: c, index: i });
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
        return Err(CallsignParseError::InvalidLength { input: input.to_string(), len: base.len() });
    }

    Ok(ParsedCallsign { base, suffix })
}