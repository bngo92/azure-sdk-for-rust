#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AclAction {
    Allow,
    Deny,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHandlerSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<serde_json::Value>,
}
impl EventHandlerSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHandlerTemplate {
    #[serde(rename = "urlTemplate")]
    pub url_template: String,
    #[serde(rename = "userEventPattern", default, skip_serializing_if = "Option::is_none")]
    pub user_event_pattern: Option<String>,
    #[serde(rename = "systemEventPattern", default, skip_serializing_if = "Option::is_none")]
    pub system_event_pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<UpstreamAuthSettings>,
}
impl EventHandlerTemplate {
    pub fn new(url_template: String) -> Self {
        Self {
            url_template,
            user_event_pattern: None,
            system_event_pattern: None,
            auth: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum KeyType {
    Primary,
    Secondary,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveTraceCategory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
}
impl LiveTraceCategory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveTraceConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<LiveTraceCategory>,
}
impl LiveTraceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentity {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagedIdentityType>,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentitySettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl ManagedIdentitySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ManagedIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailability {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl NameAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityParameters {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
}
impl NameAvailabilityParameters {
    pub fn new(type_: String, name: String) -> Self {
        Self { type_, name }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkAcl {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allow: Vec<WebPubSubRequestType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deny: Vec<WebPubSubRequestType>,
}
impl NetworkAcl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointAcl {
    #[serde(flatten)]
    pub network_acl: NetworkAcl,
    pub name: String,
}
impl PrivateEndpointAcl {
    pub fn new(name: String) -> Self {
        Self {
            network_acl: NetworkAcl::default(),
            name,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PrivateEndpointConnectionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PrivateLinkResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
    #[serde(rename = "shareablePrivateLinkResourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub shareable_private_link_resource_types: Vec<ShareablePrivateLinkResourceType>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateLinkServiceConnectionStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateLinkServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Unknown,
    Succeeded,
    Failed,
    Canceled,
    Running,
    Creating,
    Updating,
    Deleting,
    Moving,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegenerateKeyParameters {
    #[serde(rename = "keyType", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<KeyType>,
}
impl RegenerateKeyParameters {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<WebPubSubSkuTier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl ResourceSku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ScaleType {
    None,
    Manual,
    Automatic,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareablePrivateLinkResourceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ShareablePrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareablePrivateLinkResourceType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ShareablePrivateLinkResourceProperties>,
}
impl ShareablePrivateLinkResourceType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedPrivateLinkResourceProperties>,
}
impl SharedPrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SharedPrivateLinkResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SharedPrivateLinkResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedPrivateLinkResourceProperties {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "privateLinkResourceId")]
    pub private_link_resource_id: String,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "requestMessage", default, skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SharedPrivateLinkResourceStatus>,
}
impl SharedPrivateLinkResourceProperties {
    pub fn new(group_id: String, private_link_resource_id: String) -> Self {
        Self {
            group_id,
            private_link_resource_id,
            provisioning_state: None,
            request_message: None,
            status: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SharedPrivateLinkResourceStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
    Timeout,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRServiceUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SignalRServiceUsageName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl SignalRServiceUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRServiceUsageList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SignalRServiceUsage>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SignalRServiceUsageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRServiceUsageName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl SignalRServiceUsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<SkuCapacity>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapacity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_values: Vec<i32>,
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<ScaleType>,
}
impl SkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Sku>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SkuList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpstreamAuthSettings {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpstreamAuthType>,
    #[serde(rename = "managedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub managed_identity: Option<ManagedIdentitySettings>,
}
impl UpstreamAuthSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UpstreamAuthType {
    None,
    ManagedIdentity,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentityProperty {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentityProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubKeys {
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
}
impl WebPubSubKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubNetworkAcLs {
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<AclAction>,
    #[serde(rename = "publicNetwork", default, skip_serializing_if = "Option::is_none")]
    pub public_network: Option<NetworkAcl>,
    #[serde(rename = "privateEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoints: Vec<PrivateEndpointAcl>,
}
impl WebPubSubNetworkAcLs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "externalIP", default, skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "publicPort", default, skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
    #[serde(rename = "serverPort", default, skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[serde(rename = "sharedPrivateLinkResources", default, skip_serializing_if = "Vec::is_empty")]
    pub shared_private_link_resources: Vec<SharedPrivateLinkResource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<WebPubSubTlsSettings>,
    #[serde(rename = "liveTraceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub live_trace_configuration: Option<LiveTraceConfiguration>,
    #[serde(rename = "eventHandler", default, skip_serializing_if = "Option::is_none")]
    pub event_handler: Option<EventHandlerSettings>,
    #[serde(rename = "networkACLs", default, skip_serializing_if = "Option::is_none")]
    pub network_ac_ls: Option<WebPubSubNetworkAcLs>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<String>,
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
    #[serde(rename = "disableAadAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_aad_auth: Option<bool>,
}
impl WebPubSubProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WebPubSubRequestType {
    ClientConnection,
    ServerConnection,
    #[serde(rename = "RESTAPI")]
    Restapi,
    Trace,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebPubSubProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedIdentity>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl WebPubSubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WebPubSubResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl WebPubSubResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WebPubSubSkuTier {
    Free,
    Basic,
    Standard,
    Premium,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubTlsSettings {
    #[serde(rename = "clientCertEnabled", default, skip_serializing_if = "Option::is_none")]
    pub client_cert_enabled: Option<bool>,
}
impl WebPubSubTlsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
