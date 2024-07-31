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
pub struct RenameEpisodeResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "seriesId", skip_serializing_if = "Option::is_none")]
    pub series_id: Option<i32>,
    #[serde(rename = "seasonNumber", skip_serializing_if = "Option::is_none")]
    pub season_number: Option<i32>,
    #[serde(rename = "episodeNumbers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub episode_numbers: Option<Option<Vec<i32>>>,
    #[serde(rename = "episodeFileId", skip_serializing_if = "Option::is_none")]
    pub episode_file_id: Option<i32>,
    #[serde(rename = "existingPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub existing_path: Option<Option<String>>,
    #[serde(rename = "newPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub new_path: Option<Option<String>>,
}

impl RenameEpisodeResource {
    pub fn new() -> RenameEpisodeResource {
        RenameEpisodeResource {
            id: None,
            series_id: None,
            season_number: None,
            episode_numbers: None,
            episode_file_id: None,
            existing_path: None,
            new_path: None,
        }
    }
}

