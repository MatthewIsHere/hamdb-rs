//! HamDB API v1 client.
//!
//! This module exposes the [`Client`] type that performs callsign lookups against
//! the public HamDB REST API and returns strongly typed response data.

use crate::Error;
use crate::{deserialize::*, parsing::parse_callsign};
use serde::Deserialize;
use std::{borrow::Cow, time::Duration};
use time::Date;

const V1_ENDPOINT: &str = "https://api.hamdb.org/v1/";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

fn make_url(callsign: &str, app_name: &str) -> String {
    V1_ENDPOINT.to_string() + callsign + "/json/" + app_name
}

#[derive(Debug, Clone)]
/// Asynchronous HamDB API client.
///
/// Configure the client with an application identifier (per HamDB usage rules)
/// and call [`lookup`](Client::lookup) with callsigns.
///
/// # Examples
/// ```ignore
/// use hamdb::v1::Client;
///
/// # async fn example() -> Result<(), hamdb::Error> {
/// let client = Client::new("station-dashboard");
/// let lookup = client.lookup("W1AW").await?;
/// println!("{} expires {:?}", lookup.call, lookup.expires);
/// # Ok(())
/// # }
/// ```
pub struct Client {
    app_name: Cow<'static, str>,
    http_client: reqwest::Client,
}

#[derive(Debug, Clone, Deserialize)]
/// Successful callsign lookup payload returned by the HamDB API.
pub struct CallsignLookup {
    /// Callsign returned exactly as HamDB records it.
    pub call: String,
    /// FCC license class (e.g. `"E"`).
    pub class: String,
    /// License expiration date.
    #[serde(deserialize_with = "string_as_mdy")]
    pub expires: Option<Date>, // is 06/08/2028
    /// License status description.
    #[serde(deserialize_with = "empty_as_none")]
    pub status: Option<String>,
    /// Maidenhead grid square.
    pub grid: String,
    /// Approximate latitude of the station address.
    #[serde(deserialize_with = "latitude_as_f64")]
    pub lat: Option<f64>,
    /// Approximate longitude of the station address.
    #[serde(deserialize_with = "longitude_as_f64")]
    pub lon: Option<f64>,
    /// Optional first name associated with the licensee.
    #[serde(deserialize_with = "empty_as_none", rename = "fname")]
    pub first_name: Option<String>,
    /// Optional middle initial.
    #[serde(deserialize_with = "empty_as_none", rename = "mi")]
    pub middle_initial: Option<String>,
    /// Legal name returned by HamDB.
    pub name: String,
    /// Optional name suffix (Jr, Sr, etc.).
    #[serde(deserialize_with = "empty_as_none")]
    pub suffix: Option<String>,
    /// Primary street address line.
    #[serde(deserialize_with = "empty_as_none")]
    pub addr1: Option<String>,
    /// Secondary street address line (PO box, suite, etc.).
    #[serde(deserialize_with = "empty_as_none")]
    pub addr2: Option<String>,
    /// State or region abbreviation.
    #[serde(deserialize_with = "empty_as_none")]
    pub state: Option<String>,
    /// Postal ZIP code.
    #[serde(deserialize_with = "empty_as_none")]
    pub zip: Option<String>,
    /// Country stored in HamDB.
    pub country: String,
}

impl Client {
    /// Construct a new client with the application name sent to HamDB.
    ///
    /// HamDB requires callers to identify their application; the value is sent
    /// verbatim as part of the request URL.
    pub fn new(app_name: impl Into<Cow<'static, str>>) -> Self {
        let http_client = reqwest::Client::new();
        Self {
            app_name: app_name.into(),
            http_client,
        }
    }

    /// Look up a callsign via the HamDB API.
    ///
    /// The input is validated locally (uppercase, allowed characters and length)
    /// before performing the HTTP request. If the API returns an explicit
    /// `NOT_FOUND` status, [`Error::NotFound`] is produced.
    ///
    /// # Errors
    /// * [`Error::CallsignParsing`] – malformed user input.
    /// * [`Error::Http`] / [`Error::Timeout`] – networking problems.
    /// * [`Error::BodyParsing`] – response JSON could not be decoded.
    /// * [`Error::NotFound`] – HamDB could not locate the callsign.
    ///
    /// # Examples
    /// ```ignore
    /// # async fn demo() -> Result<(), hamdb::Error> {
    /// let client = hamdb::v1::Client::new("station-dashboard");
    /// match client.lookup("W4AQL").await {
    ///     Ok(info) => println!("{} lives in {:?}", info.call, info.state),
    ///     Err(hamdb::Error::NotFound(call)) => println!("{} missing", call),
    ///     Err(other) => eprintln!("lookup failed: {other}")
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn lookup(&self, callsign: &str) -> Result<CallsignLookup, Error> {
        let parsed = parse_callsign(callsign)?;
        let url = make_url(&parsed.base, &self.app_name);
        let res = self
            .http_client
            .get(url)
            .timeout(DEFAULT_TIMEOUT)
            .send()
            .await?;
        let value: ApiResponse = res.json().await?;

        if let Status::NotFound = value.hamdb.messages.status {
            return Err(Error::NotFound(parsed.base.to_string()));
        }

        return Ok(value.hamdb.callsign);
    }
}

// JSON response format

#[derive(Deserialize)]
struct ApiResponse {
    hamdb: HamDb,
}
#[derive(Deserialize)]
struct HamDb {
    // version: String,
    callsign: CallsignLookup,
    messages: Messages,
}
#[derive(Deserialize)]
struct Messages {
    status: Status,
}
#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
enum Status {
    Ok,
    NotFound,
}
