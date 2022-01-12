#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudShellConsole {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: ConsoleProperties,
}
impl CloudShellConsole {
    pub fn new(properties: ConsoleProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudShellPatchUserSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserProperties>,
}
impl CloudShellPatchUserSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudShellUserSettings {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: UserProperties,
}
impl CloudShellUserSettings {
    pub fn new(properties: UserProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsoleCreateProperties {
    #[serde(rename = "osType")]
    pub os_type: console_create_properties::OsType,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<console_create_properties::ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl ConsoleCreateProperties {
    pub fn new(os_type: console_create_properties::OsType) -> Self {
        Self {
            os_type,
            provisioning_state: None,
            uri: None,
        }
    }
}
pub mod console_create_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Linux,
        Windows,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Pending,
        Updating,
        Creating,
        Repairing,
        Failed,
        Canceled,
        Succeeded,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsoleDefinition {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: ConsoleCreateProperties,
}
impl ConsoleDefinition {
    pub fn new(properties: ConsoleCreateProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsoleProperties {
    #[serde(rename = "osType")]
    pub os_type: console_properties::OsType,
    #[serde(rename = "provisioningState")]
    pub provisioning_state: console_properties::ProvisioningState,
    pub uri: String,
}
impl ConsoleProperties {
    pub fn new(os_type: console_properties::OsType, provisioning_state: console_properties::ProvisioningState, uri: String) -> Self {
        Self {
            os_type,
            provisioning_state,
            uri,
        }
    }
}
pub mod console_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Linux,
        Windows,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Pending,
        Updating,
        Creating,
        Repairing,
        Failed,
        Canceled,
        Succeeded,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}
impl ErrorResponse {
    pub fn new(error: ErrorDetail) -> Self {
        Self { error }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[serde(rename = "fileShareName", default, skip_serializing_if = "Option::is_none")]
    pub file_share_name: Option<String>,
    #[serde(rename = "diskSizeInGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_in_gb: Option<i32>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TerminalSettings {
    #[serde(rename = "fontSize", default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<terminal_settings::FontSize>,
    #[serde(rename = "fontStyle", default, skip_serializing_if = "Option::is_none")]
    pub font_style: Option<terminal_settings::FontStyle>,
}
impl TerminalSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod terminal_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FontSize {
        NotSpecified,
        Small,
        Medium,
        Large,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FontStyle {
        NotSpecified,
        Monospace,
        Courier,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProperties {
    #[serde(rename = "preferredOsType")]
    pub preferred_os_type: user_properties::PreferredOsType,
    #[serde(rename = "preferredLocation")]
    pub preferred_location: String,
    #[serde(rename = "storageProfile")]
    pub storage_profile: StorageProfile,
    #[serde(rename = "terminalSettings")]
    pub terminal_settings: TerminalSettings,
    #[serde(rename = "preferredShellType")]
    pub preferred_shell_type: user_properties::PreferredShellType,
}
impl UserProperties {
    pub fn new(
        preferred_os_type: user_properties::PreferredOsType,
        preferred_location: String,
        storage_profile: StorageProfile,
        terminal_settings: TerminalSettings,
        preferred_shell_type: user_properties::PreferredShellType,
    ) -> Self {
        Self {
            preferred_os_type,
            preferred_location,
            storage_profile,
            terminal_settings,
            preferred_shell_type,
        }
    }
}
pub mod user_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PreferredOsType {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PreferredShellType {
        #[serde(rename = "bash")]
        Bash,
        #[serde(rename = "pwsh")]
        Pwsh,
        #[serde(rename = "powershell")]
        Powershell,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSettingsResponse {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: UserProperties,
}
impl UserSettingsResponse {
    pub fn new(properties: UserProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
