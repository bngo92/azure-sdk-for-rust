#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectivityCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<ManagedNetworkGroup>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peerings: Vec<ManagedNetworkPeeringPolicy>,
}
impl ConnectivityCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HubAndSpokePeeringPolicyProperties {
    #[serde(flatten)]
    pub managed_network_peering_policy_properties: ManagedNetworkPeeringPolicyProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hub: Option<ResourceId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spokes: Vec<ResourceId>,
}
impl HubAndSpokePeeringPolicyProperties {
    pub fn new(managed_network_peering_policy_properties: ManagedNetworkPeeringPolicyProperties) -> Self {
        Self {
            managed_network_peering_policy_properties,
            hub: None,
            spokes: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedNetworkProperties>,
}
impl ManagedNetwork {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedNetworkGroupProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<managed_network_group::Kind>,
}
impl ManagedNetworkGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_network_group {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Connectivity,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkGroupListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedNetworkGroup>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ManagedNetworkGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkGroupProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[serde(rename = "managementGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub management_groups: Vec<ResourceId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<ResourceId>,
    #[serde(rename = "virtualNetworks", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_networks: Vec<ResourceId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<ResourceId>,
}
impl ManagedNetworkGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedNetwork>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ManagedNetworkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkPeeringPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedNetworkPeeringPolicyProperties>,
}
impl ManagedNetworkPeeringPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkPeeringPolicyListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedNetworkPeeringPolicy>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ManagedNetworkPeeringPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetworkPeeringPolicyProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[serde(rename = "type")]
    pub type_: managed_network_peering_policy_properties::Type,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hub: Option<ResourceId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spokes: Vec<ResourceId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mesh: Vec<ResourceId>,
}
impl ManagedNetworkPeeringPolicyProperties {
    pub fn new(type_: managed_network_peering_policy_properties::Type) -> Self {
        Self {
            resource_properties: ResourceProperties::default(),
            type_,
            hub: None,
            spokes: Vec::new(),
            mesh: Vec::new(),
        }
    }
}
pub mod managed_network_peering_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        HubAndSpokeTopology,
        MeshTopology,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connectivity: Option<ConnectivityCollection>,
}
impl ManagedNetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ManagedNetworkUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeshPeeringPolicyProperties {
    #[serde(flatten)]
    pub managed_network_peering_policy_properties: ManagedNetworkPeeringPolicyProperties,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mesh: Vec<ResourceId>,
}
impl MeshPeeringPolicyProperties {
    pub fn new(managed_network_peering_policy_properties: ManagedNetworkPeeringPolicyProperties) -> Self {
        Self {
            managed_network_peering_policy_properties,
            mesh: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
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
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResourceId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<resource_properties::ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Updating,
        Deleting,
        Failed,
        Succeeded,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Scope {
    #[serde(rename = "managementGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub management_groups: Vec<ResourceId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<ResourceId>,
    #[serde(rename = "virtualNetworks", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_networks: Vec<ResourceId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<ResourceId>,
}
impl Scope {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScopeAssignmentProperties>,
}
impl ScopeAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeAssignmentListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScopeAssignment>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ScopeAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeAssignmentProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[serde(rename = "assignedManagedNetwork", default, skip_serializing_if = "Option::is_none")]
    pub assigned_managed_network: Option<String>,
}
impl ScopeAssignmentProperties {
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
}
impl TrackedResource {
    pub fn new() -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
        }
    }
}
