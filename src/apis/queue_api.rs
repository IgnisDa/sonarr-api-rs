/*
 * Sonarr
 *
 * Sonarr API docs - The v3 API docs apply to both v3 and v4 versions of Sonarr. Some functionality may only be available in v4 of the Sonarr application.
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`api_v3_queue_bulk_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV3QueueBulkDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v3_queue_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV3QueueGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v3_queue_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV3QueueIdDeleteError {
    UnknownValue(serde_json::Value),
}


pub async fn api_v3_queue_bulk_delete(configuration: &configuration::Configuration, remove_from_client: Option<bool>, blocklist: Option<bool>, skip_redownload: Option<bool>, change_category: Option<bool>, queue_bulk_resource: Option<models::QueueBulkResource>) -> Result<(), Error<ApiV3QueueBulkDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v3/queue/bulk", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = remove_from_client {
        local_var_req_builder = local_var_req_builder.query(&[("removeFromClient", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = blocklist {
        local_var_req_builder = local_var_req_builder.query(&[("blocklist", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = skip_redownload {
        local_var_req_builder = local_var_req_builder.query(&[("skipRedownload", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = change_category {
        local_var_req_builder = local_var_req_builder.query(&[("changeCategory", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("apikey", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Api-Key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&queue_bulk_resource);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ApiV3QueueBulkDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn api_v3_queue_get(configuration: &configuration::Configuration, page: Option<i32>, page_size: Option<i32>, sort_key: Option<&str>, sort_direction: Option<models::SortDirection>, include_unknown_series_items: Option<bool>, include_series: Option<bool>, include_episode: Option<bool>, series_ids: Option<Vec<i32>>, protocol: Option<models::DownloadProtocol>, languages: Option<Vec<i32>>, quality: Option<i32>) -> Result<models::QueueResourcePagingResource, Error<ApiV3QueueGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v3/queue", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_key {
        local_var_req_builder = local_var_req_builder.query(&[("sortKey", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_direction {
        local_var_req_builder = local_var_req_builder.query(&[("sortDirection", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_unknown_series_items {
        local_var_req_builder = local_var_req_builder.query(&[("includeUnknownSeriesItems", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_series {
        local_var_req_builder = local_var_req_builder.query(&[("includeSeries", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_episode {
        local_var_req_builder = local_var_req_builder.query(&[("includeEpisode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = series_ids {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("seriesIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("seriesIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = protocol {
        local_var_req_builder = local_var_req_builder.query(&[("protocol", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = languages {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("languages".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("languages", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = quality {
        local_var_req_builder = local_var_req_builder.query(&[("quality", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("apikey", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Api-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ApiV3QueueGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn api_v3_queue_id_delete(configuration: &configuration::Configuration, id: i32, remove_from_client: Option<bool>, blocklist: Option<bool>, skip_redownload: Option<bool>, change_category: Option<bool>) -> Result<(), Error<ApiV3QueueIdDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v3/queue/{id}", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = remove_from_client {
        local_var_req_builder = local_var_req_builder.query(&[("removeFromClient", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = blocklist {
        local_var_req_builder = local_var_req_builder.query(&[("blocklist", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = skip_redownload {
        local_var_req_builder = local_var_req_builder.query(&[("skipRedownload", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = change_category {
        local_var_req_builder = local_var_req_builder.query(&[("changeCategory", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("apikey", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Api-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ApiV3QueueIdDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

