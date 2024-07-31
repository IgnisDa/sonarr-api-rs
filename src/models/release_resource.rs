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
pub struct ReleaseResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "guid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub guid: Option<Option<String>>,
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<Box<models::QualityModel>>,
    #[serde(rename = "qualityWeight", skip_serializing_if = "Option::is_none")]
    pub quality_weight: Option<i32>,
    #[serde(rename = "age", skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(rename = "ageHours", skip_serializing_if = "Option::is_none")]
    pub age_hours: Option<f64>,
    #[serde(rename = "ageMinutes", skip_serializing_if = "Option::is_none")]
    pub age_minutes: Option<f64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "indexerId", skip_serializing_if = "Option::is_none")]
    pub indexer_id: Option<i32>,
    #[serde(rename = "indexer", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub indexer: Option<Option<String>>,
    #[serde(rename = "releaseGroup", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub release_group: Option<Option<String>>,
    #[serde(rename = "subGroup", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<Option<String>>,
    #[serde(rename = "releaseHash", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub release_hash: Option<Option<String>>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "fullSeason", skip_serializing_if = "Option::is_none")]
    pub full_season: Option<bool>,
    #[serde(rename = "sceneSource", skip_serializing_if = "Option::is_none")]
    pub scene_source: Option<bool>,
    #[serde(rename = "seasonNumber", skip_serializing_if = "Option::is_none")]
    pub season_number: Option<i32>,
    #[serde(rename = "languages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Option<Vec<models::Language>>>,
    #[serde(rename = "languageWeight", skip_serializing_if = "Option::is_none")]
    pub language_weight: Option<i32>,
    #[serde(rename = "airDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub air_date: Option<Option<String>>,
    #[serde(rename = "seriesTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_title: Option<Option<String>>,
    #[serde(rename = "episodeNumbers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub episode_numbers: Option<Option<Vec<i32>>>,
    #[serde(rename = "absoluteEpisodeNumbers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub absolute_episode_numbers: Option<Option<Vec<i32>>>,
    #[serde(rename = "mappedSeasonNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mapped_season_number: Option<Option<i32>>,
    #[serde(rename = "mappedEpisodeNumbers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mapped_episode_numbers: Option<Option<Vec<i32>>>,
    #[serde(rename = "mappedAbsoluteEpisodeNumbers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mapped_absolute_episode_numbers: Option<Option<Vec<i32>>>,
    #[serde(rename = "mappedSeriesId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mapped_series_id: Option<Option<i32>>,
    #[serde(rename = "mappedEpisodeInfo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mapped_episode_info: Option<Option<Vec<models::ReleaseEpisodeResource>>>,
    #[serde(rename = "approved", skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
    #[serde(rename = "temporarilyRejected", skip_serializing_if = "Option::is_none")]
    pub temporarily_rejected: Option<bool>,
    #[serde(rename = "rejected", skip_serializing_if = "Option::is_none")]
    pub rejected: Option<bool>,
    #[serde(rename = "tvdbId", skip_serializing_if = "Option::is_none")]
    pub tvdb_id: Option<i32>,
    #[serde(rename = "tvRageId", skip_serializing_if = "Option::is_none")]
    pub tv_rage_id: Option<i32>,
    #[serde(rename = "rejections", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rejections: Option<Option<Vec<String>>>,
    #[serde(rename = "publishDate", skip_serializing_if = "Option::is_none")]
    pub publish_date: Option<String>,
    #[serde(rename = "commentUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub comment_url: Option<Option<String>>,
    #[serde(rename = "downloadUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<Option<String>>,
    #[serde(rename = "infoUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub info_url: Option<Option<String>>,
    #[serde(rename = "episodeRequested", skip_serializing_if = "Option::is_none")]
    pub episode_requested: Option<bool>,
    #[serde(rename = "downloadAllowed", skip_serializing_if = "Option::is_none")]
    pub download_allowed: Option<bool>,
    #[serde(rename = "releaseWeight", skip_serializing_if = "Option::is_none")]
    pub release_weight: Option<i32>,
    #[serde(rename = "customFormats", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_formats: Option<Option<Vec<models::CustomFormatResource>>>,
    #[serde(rename = "customFormatScore", skip_serializing_if = "Option::is_none")]
    pub custom_format_score: Option<i32>,
    #[serde(rename = "sceneMapping", skip_serializing_if = "Option::is_none")]
    pub scene_mapping: Option<Box<models::AlternateTitleResource>>,
    #[serde(rename = "magnetUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub magnet_url: Option<Option<String>>,
    #[serde(rename = "infoHash", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub info_hash: Option<Option<String>>,
    #[serde(rename = "seeders", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub seeders: Option<Option<i32>>,
    #[serde(rename = "leechers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub leechers: Option<Option<i32>>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<models::DownloadProtocol>,
    #[serde(rename = "indexerFlags", skip_serializing_if = "Option::is_none")]
    pub indexer_flags: Option<i32>,
    #[serde(rename = "isDaily", skip_serializing_if = "Option::is_none")]
    pub is_daily: Option<bool>,
    #[serde(rename = "isAbsoluteNumbering", skip_serializing_if = "Option::is_none")]
    pub is_absolute_numbering: Option<bool>,
    #[serde(rename = "isPossibleSpecialEpisode", skip_serializing_if = "Option::is_none")]
    pub is_possible_special_episode: Option<bool>,
    #[serde(rename = "special", skip_serializing_if = "Option::is_none")]
    pub special: Option<bool>,
    #[serde(rename = "seriesId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_id: Option<Option<i32>>,
    #[serde(rename = "episodeId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub episode_id: Option<Option<i32>>,
    #[serde(rename = "episodeIds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub episode_ids: Option<Option<Vec<i32>>>,
    #[serde(rename = "downloadClientId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub download_client_id: Option<Option<i32>>,
    #[serde(rename = "downloadClient", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub download_client: Option<Option<String>>,
    #[serde(rename = "shouldOverride", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub should_override: Option<Option<bool>>,
}

impl ReleaseResource {
    pub fn new() -> ReleaseResource {
        ReleaseResource {
            id: None,
            guid: None,
            quality: None,
            quality_weight: None,
            age: None,
            age_hours: None,
            age_minutes: None,
            size: None,
            indexer_id: None,
            indexer: None,
            release_group: None,
            sub_group: None,
            release_hash: None,
            title: None,
            full_season: None,
            scene_source: None,
            season_number: None,
            languages: None,
            language_weight: None,
            air_date: None,
            series_title: None,
            episode_numbers: None,
            absolute_episode_numbers: None,
            mapped_season_number: None,
            mapped_episode_numbers: None,
            mapped_absolute_episode_numbers: None,
            mapped_series_id: None,
            mapped_episode_info: None,
            approved: None,
            temporarily_rejected: None,
            rejected: None,
            tvdb_id: None,
            tv_rage_id: None,
            rejections: None,
            publish_date: None,
            comment_url: None,
            download_url: None,
            info_url: None,
            episode_requested: None,
            download_allowed: None,
            release_weight: None,
            custom_formats: None,
            custom_format_score: None,
            scene_mapping: None,
            magnet_url: None,
            info_hash: None,
            seeders: None,
            leechers: None,
            protocol: None,
            indexer_flags: None,
            is_daily: None,
            is_absolute_numbering: None,
            is_possible_special_episode: None,
            special: None,
            series_id: None,
            episode_id: None,
            episode_ids: None,
            download_client_id: None,
            download_client: None,
            should_override: None,
        }
    }
}

