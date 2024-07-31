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
pub struct EpisodeFileListResource {
    #[serde(rename = "episodeFileIds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub episode_file_ids: Option<Option<Vec<i32>>>,
    #[serde(rename = "languages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Option<Vec<models::Language>>>,
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<Box<models::QualityModel>>,
    #[serde(rename = "sceneName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scene_name: Option<Option<String>>,
    #[serde(rename = "releaseGroup", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub release_group: Option<Option<String>>,
}

impl EpisodeFileListResource {
    pub fn new() -> EpisodeFileListResource {
        EpisodeFileListResource {
            episode_file_ids: None,
            languages: None,
            quality: None,
            scene_name: None,
            release_group: None,
        }
    }
}

