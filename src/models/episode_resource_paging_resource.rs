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
pub struct EpisodeResourcePagingResource {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "sortKey", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_key: Option<Option<String>>,
    #[serde(rename = "sortDirection", skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<models::SortDirection>,
    #[serde(rename = "totalRecords", skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i32>,
    #[serde(rename = "records", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub records: Option<Option<Vec<models::EpisodeResource>>>,
}

impl EpisodeResourcePagingResource {
    pub fn new() -> EpisodeResourcePagingResource {
        EpisodeResourcePagingResource {
            page: None,
            page_size: None,
            sort_key: None,
            sort_direction: None,
            total_records: None,
            records: None,
        }
    }
}

