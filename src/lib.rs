//! Minimal async client for the public [HamDB](https://hamdb.org) amateur-radio
//! callsign lookup service.
//!
//! The crate exposes a [`v1`] module that implements the current version of the API
//! as well as a couple of helper modules for parsing and error handling.
//!
//! # Quick start
//! ```ignore
//! use hamdb::v1::Client;
//!
//! # async fn example() -> Result<(), hamdb::Error> {
//! let client = Client::new("my-app-name");
//! let callsign = client.lookup("W4AQL").await?;
//! println!("{} ({})", callsign.call, callsign.country);
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod parsing;
pub use error::Error;
mod deserialize;

pub mod v1;
