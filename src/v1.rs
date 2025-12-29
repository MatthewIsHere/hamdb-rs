use std::{borrow::Cow, time::Duration};
use crate::{deserialize::*, parsing::parse_callsign};
use serde::Deserialize;
use time::Date;
use crate::Error;

const V1_ENDPOINT: &str = "https://api.hamdb.org/v1/";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

fn make_url(callsign: &str, app_name: &str) -> String {
    V1_ENDPOINT.to_string() + callsign + "/json/" + app_name
}

#[derive(Debug, Clone)]
pub struct Client {
    app_name: Cow<'static, str>,
    http_client: reqwest::Client,
}


#[derive(Debug, Clone, Deserialize)]
pub struct CallsignLookup {
    pub call: String,
    pub class: String,
    #[serde(deserialize_with = "string_as_mdy")]
    pub expires: Option<Date>, // is 06/08/2028
    #[serde(deserialize_with = "empty_as_none")]
    pub status: Option<String>,
    pub grid: String,
    #[serde(deserialize_with = "latitude_as_f64")]
    pub lat: Option<f64>,
    #[serde(deserialize_with = "longitude_as_f64")]
    pub lon: Option<f64>,
    #[serde(deserialize_with = "empty_as_none", rename = "fname")]
    pub first_name: Option<String>,
    #[serde(deserialize_with = "empty_as_none", rename = "mi")]
    pub middle_initial: Option<String>,
    pub name: String,
    #[serde(deserialize_with = "empty_as_none")]
    pub suffix: Option<String>,
    #[serde(deserialize_with = "empty_as_none")]
    pub addr1: Option<String>,
    #[serde(deserialize_with = "empty_as_none")]
    pub addr2: Option<String>,
    #[serde(deserialize_with = "empty_as_none")]
    pub state: Option<String>,
    #[serde(deserialize_with = "empty_as_none")]
    pub zip: Option<String>,
    pub country: String
}

impl Client {
    pub fn new(app_name: impl Into<Cow<'static, str>>) -> Self {
        let http_client = reqwest::Client::new();
        Self {
            app_name: app_name.into(),
            http_client,
        }
    }

    pub async fn lookup(&self, callsign: &str) -> Result<CallsignLookup, Error> {
        let parsed = parse_callsign(callsign)?;
        let url = make_url(&parsed.base, &self.app_name);
        let res = self.http_client.get(url)
            .timeout(DEFAULT_TIMEOUT)
            .send()
            .await?;
        let value: ApiResponse = res.json().await?;

        if let Status::NotFound = value.hamdb.messages.status {
            return Err(Error::NotFound(parsed.base.to_string()));
        }

        return Ok(value.hamdb.callsign)
    }
}


// JSON response format

#[derive(Deserialize)]
struct ApiResponse {
    hamdb: HamDb
}
#[derive(Deserialize)]
struct HamDb {
    // version: String,
    callsign: CallsignLookup,
    messages: Messages
}
#[derive(Deserialize)]
struct Messages {
    status: Status
}
#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
enum Status {
    Ok,
    NotFound
}