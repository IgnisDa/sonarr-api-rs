/*
 * Sonarr
 *
 * Sonarr API docs - The v3 API docs apply to both v3 and v4 versions of Sonarr. Some functionality may only be available in v4 of the Sonarr application.
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QualitySource {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "television")]
    Television,
    #[serde(rename = "televisionRaw")]
    TelevisionRaw,
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "webRip")]
    WebRip,
    #[serde(rename = "dvd")]
    Dvd,
    #[serde(rename = "bluray")]
    Bluray,
    #[serde(rename = "blurayRaw")]
    BlurayRaw,

}

impl std::fmt::Display for QualitySource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Television => write!(f, "television"),
            Self::TelevisionRaw => write!(f, "televisionRaw"),
            Self::Web => write!(f, "web"),
            Self::WebRip => write!(f, "webRip"),
            Self::Dvd => write!(f, "dvd"),
            Self::Bluray => write!(f, "bluray"),
            Self::BlurayRaw => write!(f, "blurayRaw"),
        }
    }
}

impl Default for QualitySource {
    fn default() -> QualitySource {
        Self::Unknown
    }
}

