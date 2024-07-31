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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quality {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<models::QualitySource>,
    #[serde(rename = "resolution", skip_serializing_if = "Option::is_none")]
    pub resolution: Option<i32>,
}

impl Quality {
    pub fn new() -> Quality {
        Quality {
            id: None,
            name: None,
            source: None,
            resolution: None,
        }
    }
}

