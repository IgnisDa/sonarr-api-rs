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
pub struct Command {
    #[serde(rename = "sendUpdatesToClient", skip_serializing_if = "Option::is_none")]
    pub send_updates_to_client: Option<bool>,
    #[serde(rename = "updateScheduledTask", skip_serializing_if = "Option::is_none")]
    pub update_scheduled_task: Option<bool>,
    #[serde(rename = "completionMessage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub completion_message: Option<Option<String>>,
    #[serde(rename = "requiresDiskAccess", skip_serializing_if = "Option::is_none")]
    pub requires_disk_access: Option<bool>,
    #[serde(rename = "isExclusive", skip_serializing_if = "Option::is_none")]
    pub is_exclusive: Option<bool>,
    #[serde(rename = "isLongRunning", skip_serializing_if = "Option::is_none")]
    pub is_long_running: Option<bool>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "lastExecutionTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_execution_time: Option<Option<String>>,
    #[serde(rename = "lastStartTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_start_time: Option<Option<String>>,
    #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<models::CommandTrigger>,
    #[serde(rename = "suppressMessages", skip_serializing_if = "Option::is_none")]
    pub suppress_messages: Option<bool>,
    #[serde(rename = "clientUserAgent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_user_agent: Option<Option<String>>,
}

impl Command {
    pub fn new() -> Command {
        Command {
            send_updates_to_client: None,
            update_scheduled_task: None,
            completion_message: None,
            requires_disk_access: None,
            is_exclusive: None,
            is_long_running: None,
            name: None,
            last_execution_time: None,
            last_start_time: None,
            trigger: None,
            suppress_messages: None,
            client_user_agent: None,
        }
    }
}

