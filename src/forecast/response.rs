//! The structure of the fields from the Carbon Intensity API /intensity/ endpoints, which are
//! required to build a `Forecast`.
use serde::Deserialize;
use time::serde::iso8601;

#[derive(Debug, Deserialize)]
pub(super) struct Body {
    pub(super) data: Data
}

#[derive(Debug, Deserialize)]
pub(super) struct Data {
    pub(super) data: Vec<Period>
}

#[derive(Debug, Deserialize)]
pub(super) struct Period {
    #[serde(with = "iso8601")]
    pub(super) from: time::OffsetDateTime,
    #[serde(with = "iso8601")]
    pub(super) to: time::OffsetDateTime,
    pub(super) intensity: Intensity
}

#[derive(Debug, Deserialize)]
pub(super) struct Intensity {
    pub(super) forecast: u16
}
