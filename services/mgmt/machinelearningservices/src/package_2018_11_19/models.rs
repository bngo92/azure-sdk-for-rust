#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aks {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<aks::Properties>,
}
impl Aks {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod aks {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "clusterFqdn", default, skip_serializing_if = "Option::is_none")]
        pub cluster_fqdn: Option<String>,
        #[serde(rename = "systemServices", default, skip_serializing_if = "Vec::is_empty")]
        pub system_services: Vec<SystemService>,
        #[serde(rename = "agentCount", default, skip_serializing_if = "Option::is_none")]
        pub agent_count: Option<i64>,
        #[serde(rename = "agentVMSize", default, skip_serializing_if = "Option::is_none")]
        pub agent_vm_size: Option<String>,
        #[serde(rename = "sslConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub ssl_configuration: Option<SslConfiguration>,
        #[serde(rename = "aksNetworkingConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub aks_networking_configuration: Option<AksNetworkingConfiguration>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AksComputeSecrets {
    #[serde(flatten)]
    pub compute_secrets: ComputeSecrets,
    #[serde(rename = "userKubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub user_kube_config: Option<String>,
    #[serde(rename = "adminKubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub admin_kube_config: Option<String>,
    #[serde(rename = "imagePullSecretName", default, skip_serializing_if = "Option::is_none")]
    pub image_pull_secret_name: Option<String>,
}
impl AksComputeSecrets {
    pub fn new(compute_secrets: ComputeSecrets) -> Self {
        Self {
            compute_secrets,
            user_kube_config: None,
            admin_kube_config: None,
            image_pull_secret_name: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AksNetworkingConfiguration {
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "serviceCidr", default, skip_serializing_if = "Option::is_none")]
    pub service_cidr: Option<String>,
    #[serde(rename = "dnsServiceIP", default, skip_serializing_if = "Option::is_none")]
    pub dns_service_ip: Option<String>,
    #[serde(rename = "dockerBridgeCidr", default, skip_serializing_if = "Option::is_none")]
    pub docker_bridge_cidr: Option<String>,
}
impl AksNetworkingConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmlCompute {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<aml_compute::Properties>,
}
impl AmlCompute {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod aml_compute {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
        pub vm_size: Option<String>,
        #[serde(rename = "vmPriority", default, skip_serializing_if = "Option::is_none")]
        pub vm_priority: Option<properties::VmPriority>,
        #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
        pub scale_settings: Option<ScaleSettings>,
        #[serde(rename = "userAccountCredentials", default, skip_serializing_if = "Option::is_none")]
        pub user_account_credentials: Option<UserAccountCredentials>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subnet: Option<ResourceId>,
        #[serde(rename = "allocationState", default, skip_serializing_if = "Option::is_none")]
        pub allocation_state: Option<properties::AllocationState>,
        #[serde(rename = "allocationStateTransitionTime", default, skip_serializing_if = "Option::is_none")]
        pub allocation_state_transition_time: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub errors: Vec<MachineLearningServiceError>,
        #[serde(rename = "currentNodeCount", default, skip_serializing_if = "Option::is_none")]
        pub current_node_count: Option<i32>,
        #[serde(rename = "targetNodeCount", default, skip_serializing_if = "Option::is_none")]
        pub target_node_count: Option<i32>,
        #[serde(rename = "nodeStateCounts", default, skip_serializing_if = "Option::is_none")]
        pub node_state_counts: Option<NodeStateCounts>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum VmPriority {
            Dedicated,
            LowPriority,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum AllocationState {
            Steady,
            Resizing,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmlComputeNodeInformation {
    #[serde(rename = "nodeId", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<f64>,
}
impl AmlComputeNodeInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmlComputeNodesInformation {
    #[serde(flatten)]
    pub compute_nodes_information: ComputeNodesInformation,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<AmlComputeNodeInformation>,
}
impl AmlComputeNodesInformation {
    pub fn new(compute_nodes_information: ComputeNodesInformation) -> Self {
        Self {
            compute_nodes_information,
            nodes: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterUpdateProperties>,
}
impl ClusterUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdateProperties {
    #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub scale_settings: Option<ScaleSettings>,
}
impl ClusterUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Compute {
    #[serde(rename = "computeType")]
    pub compute_type: ComputeType,
    #[serde(rename = "computeLocation", default, skip_serializing_if = "Option::is_none")]
    pub compute_location: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<compute::ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdOn", default, skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "modifiedOn", default, skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "provisioningErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub provisioning_errors: Vec<MachineLearningServiceError>,
    #[serde(rename = "isAttachedCompute", default, skip_serializing_if = "Option::is_none")]
    pub is_attached_compute: Option<bool>,
}
impl Compute {
    pub fn new(compute_type: ComputeType) -> Self {
        Self {
            compute_type,
            compute_location: None,
            provisioning_state: None,
            description: None,
            created_on: None,
            modified_on: None,
            resource_id: None,
            provisioning_errors: Vec::new(),
            is_attached_compute: None,
        }
    }
}
pub mod compute {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Unknown,
        Updating,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeNodesInformation {
    #[serde(rename = "computeType")]
    pub compute_type: ComputeType,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ComputeNodesInformation {
    pub fn new(compute_type: ComputeType) -> Self {
        Self {
            compute_type,
            next_link: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Compute>,
}
impl ComputeResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeSecrets {
    #[serde(rename = "computeType")]
    pub compute_type: ComputeType,
}
impl ComputeSecrets {
    pub fn new(compute_type: ComputeType) -> Self {
        Self { compute_type }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputeType {
    #[serde(rename = "AKS")]
    Aks,
    AmlCompute,
    DataFactory,
    VirtualMachine,
    #[serde(rename = "HDInsight")]
    HdInsight,
    Databricks,
    DataLakeAnalytics,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataFactory {
    #[serde(flatten)]
    pub compute: Compute,
}
impl DataFactory {
    pub fn new(compute: Compute) -> Self {
        Self { compute }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalytics {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<data_lake_analytics::Properties>,
}
impl DataLakeAnalytics {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod data_lake_analytics {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "dataLakeStoreAccountName", default, skip_serializing_if = "Option::is_none")]
        pub data_lake_store_account_name: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Databricks {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<databricks::Properties>,
}
impl Databricks {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod databricks {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "databricksAccessToken", default, skip_serializing_if = "Option::is_none")]
        pub databricks_access_token: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabricksComputeSecrets {
    #[serde(flatten)]
    pub compute_secrets: ComputeSecrets,
    #[serde(rename = "databricksAccessToken", default, skip_serializing_if = "Option::is_none")]
    pub databricks_access_token: Option<String>,
}
impl DatabricksComputeSecrets {
    pub fn new(compute_secrets: ComputeSecrets) -> Self {
        Self {
            compute_secrets,
            databricks_access_token: None,
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HdInsight {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<hd_insight::Properties>,
}
impl HdInsight {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod hd_insight {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "sshPort", default, skip_serializing_if = "Option::is_none")]
        pub ssh_port: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(rename = "administratorAccount", default, skip_serializing_if = "Option::is_none")]
        pub administrator_account: Option<VirtualMachineSshCredentials>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListUsagesResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ListUsagesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListWorkspaceKeysResult {
    #[serde(rename = "userStorageKey", default, skip_serializing_if = "Option::is_none")]
    pub user_storage_key: Option<String>,
    #[serde(rename = "userStorageResourceId", default, skip_serializing_if = "Option::is_none")]
    pub user_storage_resource_id: Option<String>,
    #[serde(rename = "appInsightsInstrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_instrumentation_key: Option<String>,
    #[serde(rename = "containerRegistryCredentials", default, skip_serializing_if = "Option::is_none")]
    pub container_registry_credentials: Option<RegistryListCredentialsResult>,
}
impl ListWorkspaceKeysResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineLearningServiceError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl MachineLearningServiceError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeStateCounts {
    #[serde(rename = "idleNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub idle_node_count: Option<i32>,
    #[serde(rename = "runningNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub running_node_count: Option<i32>,
    #[serde(rename = "preparingNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub preparing_node_count: Option<i32>,
    #[serde(rename = "unusableNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub unusable_node_count: Option<i32>,
    #[serde(rename = "leavingNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub leaving_node_count: Option<i32>,
    #[serde(rename = "preemptedNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub preempted_node_count: Option<i32>,
}
impl NodeStateCounts {
    pub fn new() -> Self {
        Self::default()
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
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedComputeResourcesList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ComputeResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PaginatedComputeResourcesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Password {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Password {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryListCredentialsResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub passwords: Vec<Password>,
}
impl RegistryListCredentialsResult {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceId {
    pub id: String,
}
impl ResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleSettings {
    #[serde(rename = "maxNodeCount")]
    pub max_node_count: i64,
    #[serde(rename = "minNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub min_node_count: Option<i64>,
    #[serde(rename = "nodeIdleTimeBeforeScaleDown", default, skip_serializing_if = "Option::is_none")]
    pub node_idle_time_before_scale_down: Option<String>,
}
impl ScaleSettings {
    pub fn new(max_node_count: i64) -> Self {
        Self {
            max_node_count,
            min_node_count: None,
            node_idle_time_before_scale_down: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalCredentials {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
}
impl ServicePrincipalCredentials {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self { client_id, client_secret }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SslConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ssl_configuration::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
}
impl SslConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ssl_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Disabled,
        Enabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemService {
    #[serde(rename = "systemServiceType", default, skip_serializing_if = "Option::is_none")]
    pub system_service_type: Option<String>,
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SystemService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<usage::Unit>,
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccountCredentials {
    #[serde(rename = "adminUserName")]
    pub admin_user_name: String,
    #[serde(rename = "adminUserSshPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_ssh_public_key: Option<String>,
    #[serde(rename = "adminUserPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_password: Option<String>,
}
impl UserAccountCredentials {
    pub fn new(admin_user_name: String) -> Self {
        Self {
            admin_user_name,
            admin_user_ssh_public_key: None,
            admin_user_password: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<virtual_machine::Properties>,
}
impl VirtualMachine {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod virtual_machine {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "virtualMachineSize", default, skip_serializing_if = "Option::is_none")]
        pub virtual_machine_size: Option<String>,
        #[serde(rename = "sshPort", default, skip_serializing_if = "Option::is_none")]
        pub ssh_port: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(rename = "administratorAccount", default, skip_serializing_if = "Option::is_none")]
        pub administrator_account: Option<VirtualMachineSshCredentials>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineSecrets {
    #[serde(flatten)]
    pub compute_secrets: ComputeSecrets,
    #[serde(rename = "administratorAccount", default, skip_serializing_if = "Option::is_none")]
    pub administrator_account: Option<VirtualMachineSshCredentials>,
}
impl VirtualMachineSecrets {
    pub fn new(compute_secrets: ComputeSecrets) -> Self {
        Self {
            compute_secrets,
            administrator_account: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSize {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "vCPUs", default, skip_serializing_if = "Option::is_none")]
    pub v_cp_us: Option<i32>,
    #[serde(rename = "osVhdSizeMB", default, skip_serializing_if = "Option::is_none")]
    pub os_vhd_size_mb: Option<i32>,
    #[serde(rename = "maxResourceVolumeMB", default, skip_serializing_if = "Option::is_none")]
    pub max_resource_volume_mb: Option<i32>,
    #[serde(rename = "memoryGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_gb: Option<f64>,
    #[serde(rename = "lowPriorityCapable", default, skip_serializing_if = "Option::is_none")]
    pub low_priority_capable: Option<bool>,
    #[serde(rename = "premiumIO", default, skip_serializing_if = "Option::is_none")]
    pub premium_io: Option<bool>,
}
impl VirtualMachineSize {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSizeListResult {
    #[serde(rename = "amlCompute", default, skip_serializing_if = "Vec::is_empty")]
    pub aml_compute: Vec<VirtualMachineSize>,
}
impl VirtualMachineSizeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSshCredentials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "publicKeyData", default, skip_serializing_if = "Option::is_none")]
    pub public_key_data: Option<String>,
    #[serde(rename = "privateKeyData", default, skip_serializing_if = "Option::is_none")]
    pub private_key_data: Option<String>,
}
impl VirtualMachineSshCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Workspace {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}
impl Workspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl WorkspaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceProperties {
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<String>,
    #[serde(rename = "applicationInsights", default, skip_serializing_if = "Option::is_none")]
    pub application_insights: Option<String>,
    #[serde(rename = "containerRegistry", default, skip_serializing_if = "Option::is_none")]
    pub container_registry: Option<String>,
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<String>,
    #[serde(rename = "discoveryUrl", default, skip_serializing_if = "Option::is_none")]
    pub discovery_url: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workspace_properties::ProvisioningState>,
}
impl WorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Unknown,
        Updating,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePropertiesUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl WorkspacePropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspacePropertiesUpdateParameters>,
}
impl WorkspaceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
