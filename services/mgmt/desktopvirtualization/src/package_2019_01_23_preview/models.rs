#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: ApplicationProperties,
}
impl Application {
    pub fn new(properties: ApplicationProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGroup {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: ApplicationGroupProperties,
}
impl ApplicationGroup {
    pub fn new(tracked_resource: TrackedResource, properties: ApplicationGroupProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationGroup>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ApplicationGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGroupPatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationGroupPatchProperties>,
}
impl ApplicationGroupPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGroupPatchProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl ApplicationGroupPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGroupProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "hostPoolArmPath")]
    pub host_pool_arm_path: String,
    #[serde(rename = "workspaceArmPath", default, skip_serializing_if = "Option::is_none")]
    pub workspace_arm_path: Option<String>,
    #[serde(rename = "applicationGroupType")]
    pub application_group_type: application_group_properties::ApplicationGroupType,
}
impl ApplicationGroupProperties {
    pub fn new(host_pool_arm_path: String, application_group_type: application_group_properties::ApplicationGroupType) -> Self {
        Self {
            description: None,
            friendly_name: None,
            host_pool_arm_path,
            workspace_arm_path: None,
            application_group_type,
        }
    }
}
pub mod application_group_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ApplicationGroupType {
        RemoteApp,
        Desktop,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ApplicationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationPatchProperties>,
}
impl ApplicationPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPatchProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "commandLineSetting", default, skip_serializing_if = "Option::is_none")]
    pub command_line_setting: Option<application_patch_properties::CommandLineSetting>,
    #[serde(rename = "commandLineArguments", default, skip_serializing_if = "Option::is_none")]
    pub command_line_arguments: Option<String>,
    #[serde(rename = "showInPortal", default, skip_serializing_if = "Option::is_none")]
    pub show_in_portal: Option<bool>,
    #[serde(rename = "iconPath", default, skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    #[serde(rename = "iconIndex", default, skip_serializing_if = "Option::is_none")]
    pub icon_index: Option<i64>,
}
impl ApplicationPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod application_patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CommandLineSetting {
        DoNotAllow,
        Allow,
        Require,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "commandLineSetting")]
    pub command_line_setting: application_properties::CommandLineSetting,
    #[serde(rename = "commandLineArguments", default, skip_serializing_if = "Option::is_none")]
    pub command_line_arguments: Option<String>,
    #[serde(rename = "showInPortal", default, skip_serializing_if = "Option::is_none")]
    pub show_in_portal: Option<bool>,
    #[serde(rename = "iconPath", default, skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    #[serde(rename = "iconIndex", default, skip_serializing_if = "Option::is_none")]
    pub icon_index: Option<i64>,
    #[serde(rename = "iconHash", default, skip_serializing_if = "Option::is_none")]
    pub icon_hash: Option<String>,
    #[serde(rename = "iconContent", default, skip_serializing_if = "Option::is_none")]
    pub icon_content: Option<String>,
}
impl ApplicationProperties {
    pub fn new(command_line_setting: application_properties::CommandLineSetting) -> Self {
        Self {
            description: None,
            friendly_name: None,
            file_path: None,
            command_line_setting,
            command_line_arguments: None,
            show_in_portal: None,
            icon_path: None,
            icon_index: None,
            icon_hash: None,
            icon_content: None,
        }
    }
}
pub mod application_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CommandLineSetting {
        DoNotAllow,
        Allow,
        Require,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Desktop {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DesktopProperties>,
}
impl Desktop {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DesktopList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Desktop>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DesktopList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DesktopPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DesktopPatchProperties>,
}
impl DesktopPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DesktopPatchProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl DesktopPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DesktopProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "iconHash", default, skip_serializing_if = "Option::is_none")]
    pub icon_hash: Option<String>,
    #[serde(rename = "iconContent", default, skip_serializing_if = "Option::is_none")]
    pub icon_content: Option<String>,
}
impl DesktopProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostPool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: HostPoolProperties,
}
impl HostPool {
    pub fn new(tracked_resource: TrackedResource, properties: HostPoolProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostPoolList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HostPool>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl HostPoolList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostPoolPatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HostPoolPatchProperties>,
}
impl HostPoolPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostPoolPatchProperties {
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "customRdpProperty", default, skip_serializing_if = "Option::is_none")]
    pub custom_rdp_property: Option<String>,
    #[serde(rename = "maxSessionLimit", default, skip_serializing_if = "Option::is_none")]
    pub max_session_limit: Option<i64>,
    #[serde(rename = "personalDesktopAssignmentType", default, skip_serializing_if = "Option::is_none")]
    pub personal_desktop_assignment_type: Option<host_pool_patch_properties::PersonalDesktopAssignmentType>,
    #[serde(rename = "loadBalancerType", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_type: Option<host_pool_patch_properties::LoadBalancerType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ring: Option<i64>,
    #[serde(rename = "validationEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub validation_environment: Option<bool>,
    #[serde(rename = "registrationInfo", default, skip_serializing_if = "Option::is_none")]
    pub registration_info: Option<RegistrationInfoPatch>,
    #[serde(rename = "ssoContext", default, skip_serializing_if = "Option::is_none")]
    pub sso_context: Option<String>,
    #[serde(rename = "preferredAppGroupType", default, skip_serializing_if = "Option::is_none")]
    pub preferred_app_group_type: Option<host_pool_patch_properties::PreferredAppGroupType>,
}
impl HostPoolPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod host_pool_patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PersonalDesktopAssignmentType {
        Automatic,
        Direct,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LoadBalancerType {
        BreadthFirst,
        DepthFirst,
        Persistent,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PreferredAppGroupType {
        None,
        Desktop,
        RailApplications,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostPoolProperties {
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "hostPoolType")]
    pub host_pool_type: host_pool_properties::HostPoolType,
    #[serde(rename = "personalDesktopAssignmentType", default, skip_serializing_if = "Option::is_none")]
    pub personal_desktop_assignment_type: Option<host_pool_properties::PersonalDesktopAssignmentType>,
    #[serde(rename = "customRdpProperty", default, skip_serializing_if = "Option::is_none")]
    pub custom_rdp_property: Option<String>,
    #[serde(rename = "maxSessionLimit", default, skip_serializing_if = "Option::is_none")]
    pub max_session_limit: Option<i64>,
    #[serde(rename = "loadBalancerType")]
    pub load_balancer_type: host_pool_properties::LoadBalancerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ring: Option<i64>,
    #[serde(rename = "validationEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub validation_environment: Option<bool>,
    #[serde(rename = "registrationInfo", default, skip_serializing_if = "Option::is_none")]
    pub registration_info: Option<RegistrationInfo>,
    #[serde(rename = "vmTemplate", default, skip_serializing_if = "Option::is_none")]
    pub vm_template: Option<String>,
    #[serde(rename = "applicationGroupReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub application_group_references: Vec<String>,
    #[serde(rename = "ssoContext", default, skip_serializing_if = "Option::is_none")]
    pub sso_context: Option<String>,
    #[serde(rename = "preferredAppGroupType")]
    pub preferred_app_group_type: host_pool_properties::PreferredAppGroupType,
}
impl HostPoolProperties {
    pub fn new(
        host_pool_type: host_pool_properties::HostPoolType,
        load_balancer_type: host_pool_properties::LoadBalancerType,
        preferred_app_group_type: host_pool_properties::PreferredAppGroupType,
    ) -> Self {
        Self {
            friendly_name: None,
            description: None,
            host_pool_type,
            personal_desktop_assignment_type: None,
            custom_rdp_property: None,
            max_session_limit: None,
            load_balancer_type,
            ring: None,
            validation_environment: None,
            registration_info: None,
            vm_template: None,
            application_group_references: Vec::new(),
            sso_context: None,
            preferred_app_group_type,
        }
    }
}
pub mod host_pool_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HostPoolType {
        Personal,
        Shared,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PersonalDesktopAssignmentType {
        Automatic,
        Direct,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LoadBalancerType {
        BreadthFirst,
        DepthFirst,
        Persistent,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PreferredAppGroupType {
        None,
        Desktop,
        RailApplications,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationInfo {
    #[serde(rename = "expirationTime", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "resetToken", default, skip_serializing_if = "Option::is_none")]
    pub reset_token: Option<bool>,
}
impl RegistrationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationInfoPatch {
    #[serde(rename = "resetToken", default, skip_serializing_if = "Option::is_none")]
    pub reset_token: Option<bool>,
}
impl RegistrationInfoPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_provider_operation::Display>,
}
impl ResourceProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider_operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperation>,
}
impl ResourceProviderOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SendMessage {
    #[serde(rename = "messageTitle", default, skip_serializing_if = "Option::is_none")]
    pub message_title: Option<String>,
    #[serde(rename = "messageBody", default, skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
}
impl SendMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHost {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SessionHostProperties>,
}
impl SessionHost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SessionHost>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SessionHostList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostPatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SessionHostPatchProperties>,
}
impl SessionHostPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostPatchProperties {
    #[serde(rename = "allowNewSession", default, skip_serializing_if = "Option::is_none")]
    pub allow_new_session: Option<bool>,
    #[serde(rename = "assignedUser", default, skip_serializing_if = "Option::is_none")]
    pub assigned_user: Option<String>,
}
impl SessionHostPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostProperties {
    #[serde(rename = "lastHeartBeat", default, skip_serializing_if = "Option::is_none")]
    pub last_heart_beat: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<i64>,
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "allowNewSession", default, skip_serializing_if = "Option::is_none")]
    pub allow_new_session: Option<bool>,
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "assignedUser", default, skip_serializing_if = "Option::is_none")]
    pub assigned_user: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<session_host_properties::Status>,
    #[serde(rename = "statusTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub status_timestamp: Option<String>,
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "sxSStackVersion", default, skip_serializing_if = "Option::is_none")]
    pub sx_s_stack_version: Option<String>,
    #[serde(rename = "updateState", default, skip_serializing_if = "Option::is_none")]
    pub update_state: Option<session_host_properties::UpdateState>,
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(rename = "updateErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub update_error_message: Option<String>,
}
impl SessionHostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod session_host_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Available,
        Unavailable,
        Shutdown,
        Disconnected,
        Upgrading,
        UpgradeFailed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpdateState {
        Initial,
        Pending,
        Started,
        Succeeded,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StartMenuItem {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StartMenuItemProperties>,
}
impl StartMenuItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StartMenuItemList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StartMenuItem>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl StartMenuItemList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StartMenuItemProperties {
    #[serde(rename = "appAlias", default, skip_serializing_if = "Option::is_none")]
    pub app_alias: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "commandLineArguments", default, skip_serializing_if = "Option::is_none")]
    pub command_line_arguments: Option<String>,
    #[serde(rename = "iconPath", default, skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    #[serde(rename = "iconIndex", default, skip_serializing_if = "Option::is_none")]
    pub icon_index: Option<i64>,
}
impl StartMenuItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserSession {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserSessionProperties>,
}
impl UserSession {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserSessionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UserSession>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl UserSessionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserSessionProperties {
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "applicationType", default, skip_serializing_if = "Option::is_none")]
    pub application_type: Option<user_session_properties::ApplicationType>,
    #[serde(rename = "sessionState", default, skip_serializing_if = "Option::is_none")]
    pub session_state: Option<user_session_properties::SessionState>,
    #[serde(rename = "activeDirectoryUserName", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_user_name: Option<String>,
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}
impl UserSessionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod user_session_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ApplicationType {
        RemoteApp,
        Desktop,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SessionState {
        Unknown,
        Active,
        Disconnected,
        Pending,
        LogOff,
        UserProfileDiskMounted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}
impl Workspace {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl WorkspaceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspacePatchProperties>,
}
impl WorkspacePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePatchProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "applicationGroupReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub application_group_references: Vec<String>,
}
impl WorkspacePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "applicationGroupReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub application_group_references: Vec<String>,
}
impl WorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
