use serde::{Deserialize, Deserializer};
use time::{Date, macros::format_description};

pub(crate) fn empty_as_none<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

pub(crate) fn lat_lon_string_as_f64<'de, D: Deserializer<'de>>(
    deserializer: D,
    min: f64,
    max: f64,
) -> Result<Option<f64>, D::Error> {
    let s = String::deserialize(deserializer)?;
    let s = s.trim();

    if s.is_empty() {
        return Ok(None);
    }

    match s.parse::<f64>() {
        Ok(v) if (min..=max).contains(&v) => Ok(Some(v)),
        _ => Ok(None),
    }
}

pub(crate) fn latitude_as_f64<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<f64>, D::Error> {
    lat_lon_string_as_f64(deserializer, -90.0, 90.0)
}

pub(crate) fn longitude_as_f64<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<f64>, D::Error> {
    lat_lon_string_as_f64(deserializer, -180.0, 180.0)
}

pub(crate) fn string_as_mdy<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Date>, D::Error> {
    let s = String::deserialize(deserializer)?;
    let s = s.trim();
    if s.is_empty() {
        return Ok(None);
    }
    let format = format_description!("[month]/[day]/[year]");
    let date = Date::parse(s, format).ok();
    return Ok(date);
}
