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
pub struct HealthResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "source", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub source: Option<Option<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::HealthCheckResult>,
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message: Option<Option<String>>,
    #[serde(rename = "wikiUrl", skip_serializing_if = "Option::is_none")]
    pub wiki_url: Option<Box<models::HttpUri>>,
}

impl HealthResource {
    pub fn new() -> HealthResource {
        HealthResource {
            id: None,
            source: None,
            r#type: None,
            message: None,
            wiki_url: None,
        }
    }
}

