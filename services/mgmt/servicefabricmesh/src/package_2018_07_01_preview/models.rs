#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "debugParams", default, skip_serializing_if = "Option::is_none")]
    pub debug_params: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<ServiceResourceDescription>,
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[serde(rename = "unhealthyEvaluation", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<application_properties::Status>,
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "serviceNames", default, skip_serializing_if = "Vec::is_empty")]
    pub service_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsDescription>,
}
impl ApplicationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod application_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Invalid,
        Ready,
        Upgrading,
        Creating,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: ApplicationResourceProperties,
}
impl ApplicationResourceDescription {
    pub fn new(tracked_resource: TrackedResource, properties: ApplicationResourceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationResourceDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationResourceDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ApplicationResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub application_properties: ApplicationProperties,
}
impl ApplicationResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AvailableOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureInternalMonitoringPipelineSinkDescription {
    #[serde(flatten)]
    pub diagnostics_sink_properties: DiagnosticsSinkProperties,
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "maConfigUrl", default, skip_serializing_if = "Option::is_none")]
    pub ma_config_url: Option<String>,
    #[serde(rename = "fluentdConfigUrl", default, skip_serializing_if = "Option::is_none")]
    pub fluentd_config_url: Option<serde_json::Value>,
    #[serde(rename = "autoKeyConfigUrl", default, skip_serializing_if = "Option::is_none")]
    pub auto_key_config_url: Option<String>,
}
impl AzureInternalMonitoringPipelineSinkDescription {
    pub fn new(diagnostics_sink_properties: DiagnosticsSinkProperties) -> Self {
        Self {
            diagnostics_sink_properties,
            account_name: None,
            namespace: None,
            ma_config_url: None,
            fluentd_config_url: None,
            auto_key_config_url: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerCodePackageProperties {
    pub name: String,
    pub image: String,
    #[serde(rename = "imageRegistryCredential", default, skip_serializing_if = "Option::is_none")]
    pub image_registry_credential: Option<ImageRegistryCredential>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<String>,
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentVariable>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub settings: Vec<Setting>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<ContainerLabel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<EndpointProperties>,
    pub resources: ResourceRequirements,
    #[serde(rename = "volumeRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_refs: Vec<ContainerVolume>,
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<ContainerInstanceView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsRef>,
}
impl ContainerCodePackageProperties {
    pub fn new(name: String, image: String, resources: ResourceRequirements) -> Self {
        Self {
            name,
            image,
            image_registry_credential: None,
            entrypoint: None,
            commands: Vec::new(),
            environment_variables: Vec::new(),
            settings: Vec::new(),
            labels: Vec::new(),
            endpoints: Vec::new(),
            resources,
            volume_refs: Vec::new(),
            instance_view: None,
            diagnostics: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "firstTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
    #[serde(rename = "lastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ContainerEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerInstanceView {
    #[serde(rename = "restartCount", default, skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i64>,
    #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<ContainerState>,
    #[serde(rename = "previousState", default, skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<ContainerState>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<ContainerEvent>,
}
impl ContainerInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerLabel {
    pub name: String,
    pub value: String,
}
impl ContainerLabel {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerLogs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl ContainerLogs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<String>,
    #[serde(rename = "finishTime", default, skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    #[serde(rename = "detailStatus", default, skip_serializing_if = "Option::is_none")]
    pub detail_status: Option<String>,
}
impl ContainerState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerVolume {
    pub name: String,
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "destinationPath")]
    pub destination_path: String,
}
impl ContainerVolume {
    pub fn new(name: String, destination_path: String) -> Self {
        Self {
            name,
            read_only: None,
            destination_path,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticsDescription {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sinks: Vec<DiagnosticsSinkProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "defaultSinkRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub default_sink_refs: Vec<String>,
}
impl DiagnosticsDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticsRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "sinkRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub sink_refs: Vec<String>,
}
impl DiagnosticsRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiagnosticsSinkKind {
    Invalid,
    AzureInternalMonitoringPipeline,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsSinkProperties {
    pub kind: DiagnosticsSinkKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl DiagnosticsSinkProperties {
    pub fn new(kind: DiagnosticsSinkKind) -> Self {
        Self {
            kind,
            name: None,
            description: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointProperties {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}
impl EndpointProperties {
    pub fn new(name: String) -> Self {
        Self { name, port: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentVariable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl EnvironmentVariable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HealthState {
    Invalid,
    Ok,
    Warning,
    Error,
    Unknown,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageRegistryCredential {
    pub server: String,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ImageRegistryCredential {
    pub fn new(server: String, username: String) -> Self {
        Self {
            server,
            username,
            password: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngressConfig {
    #[serde(rename = "qosLevel", default, skip_serializing_if = "Option::is_none")]
    pub qos_level: Option<ingress_config::QosLevel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layer4: Vec<Layer4IngressConfig>,
    #[serde(rename = "publicIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
}
impl IngressConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ingress_config {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QosLevel {
        Bronze,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Layer4IngressConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publicPort", default, skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i64>,
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "endpointName", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
}
impl Layer4IngressConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedProxyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ManagedProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "addressPrefix")]
    pub address_prefix: String,
    #[serde(rename = "ingressConfig", default, skip_serializing_if = "Option::is_none")]
    pub ingress_config: Option<IngressConfig>,
}
impl NetworkProperties {
    pub fn new(address_prefix: String) -> Self {
        Self {
            description: None,
            address_prefix,
            ingress_config: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl NetworkRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: NetworkResourceProperties,
}
impl NetworkResourceDescription {
    pub fn new(tracked_resource: TrackedResource, properties: NetworkResourceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkResourceDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkResourceDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl NetworkResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub network_properties: NetworkProperties,
}
impl NetworkResourceProperties {
    pub fn new(network_properties: NetworkProperties) -> Self {
        Self {
            provisioned_resource_properties: ProvisionedResourceProperties::default(),
            network_properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResult>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<AvailableOperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedResourceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ProvisionedResourceProperties {
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
pub struct ResourceLimits {
    #[serde(rename = "memoryInGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_gb: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
}
impl ResourceLimits {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequests {
    #[serde(rename = "memoryInGB")]
    pub memory_in_gb: f64,
    pub cpu: f64,
}
impl ResourceRequests {
    pub fn new(memory_in_gb: f64, cpu: f64) -> Self {
        Self { memory_in_gb, cpu }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub requests: ResourceRequests,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ResourceLimits>,
}
impl ResourceRequirements {
    pub fn new(requests: ResourceRequests) -> Self {
        Self { requests, limits: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceResourceDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ServiceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceReplicaDescription {
    #[serde(flatten)]
    pub service_replica_properties: ServiceReplicaProperties,
    #[serde(rename = "replicaName", default, skip_serializing_if = "Option::is_none")]
    pub replica_name: Option<String>,
}
impl ServiceReplicaDescription {
    pub fn new(service_replica_properties: ServiceReplicaProperties) -> Self {
        Self {
            service_replica_properties,
            replica_name: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceReplicaList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceReplicaDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ServiceReplicaList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceReplicaProperties {
    #[serde(rename = "osType")]
    pub os_type: service_replica_properties::OsType,
    #[serde(rename = "codePackages")]
    pub code_packages: Vec<ContainerCodePackageProperties>,
    #[serde(rename = "networkRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub network_refs: Vec<NetworkRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsRef>,
}
impl ServiceReplicaProperties {
    pub fn new(os_type: service_replica_properties::OsType, code_packages: Vec<ContainerCodePackageProperties>) -> Self {
        Self {
            os_type,
            code_packages,
            network_refs: Vec::new(),
            diagnostics: None,
        }
    }
}
pub mod service_replica_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Linux,
        Windows,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResourceDescription {
    #[serde(flatten)]
    pub managed_proxy_resource: ManagedProxyResource,
    pub properties: ServiceResourceProperties,
}
impl ServiceResourceDescription {
    pub fn new(properties: ServiceResourceProperties) -> Self {
        Self {
            managed_proxy_resource: ManagedProxyResource::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResourceProperties {
    #[serde(flatten)]
    pub service_replica_properties: ServiceReplicaProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "replicaCount", default, skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i64>,
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<service_resource_properties::Status>,
}
impl ServiceResourceProperties {
    pub fn new(service_replica_properties: ServiceReplicaProperties) -> Self {
        Self {
            service_replica_properties,
            description: None,
            replica_count: None,
            health_state: None,
            status: None,
        }
    }
}
pub mod service_resource_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Unknown,
        Active,
        Upgrading,
        Deleting,
        Creating,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Setting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Setting {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub provider: volume_properties::Provider,
    #[serde(rename = "azureFileParameters", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_parameters: Option<VolumeProviderParametersAzureFile>,
}
impl VolumeProperties {
    pub fn new(provider: volume_properties::Provider) -> Self {
        Self {
            description: None,
            provider,
            azure_file_parameters: None,
        }
    }
}
pub mod volume_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Provider {
        #[serde(rename = "SFAzureFile")]
        SfAzureFile,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProviderParametersAzureFile {
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
    #[serde(rename = "shareName")]
    pub share_name: String,
}
impl VolumeProviderParametersAzureFile {
    pub fn new(account_name: String, share_name: String) -> Self {
        Self {
            account_name,
            account_key: None,
            share_name,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: VolumeResourceProperties,
}
impl VolumeResourceDescription {
    pub fn new(tracked_resource: TrackedResource, properties: VolumeResourceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeResourceDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VolumeResourceDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl VolumeResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub volume_properties: VolumeProperties,
}
impl VolumeResourceProperties {
    pub fn new(volume_properties: VolumeProperties) -> Self {
        Self {
            provisioned_resource_properties: ProvisionedResourceProperties::default(),
            volume_properties,
        }
    }
}
