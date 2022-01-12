#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileVolume {
    #[serde(rename = "shareName")]
    pub share_name: String,
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[serde(rename = "storageAccountKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_key: Option<String>,
}
impl AzureFileVolume {
    pub fn new(share_name: String, storage_account_name: String) -> Self {
        Self {
            share_name,
            read_only: None,
            storage_account_name,
            storage_account_key: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub properties: ContainerProperties,
}
impl Container {
    pub fn new(name: String, properties: ContainerProperties) -> Self {
        Self { name, properties }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerExec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
}
impl ContainerExec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerExecRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "terminalSize", default, skip_serializing_if = "Option::is_none")]
    pub terminal_size: Option<container_exec_request::TerminalSize>,
}
impl ContainerExecRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod container_exec_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TerminalSize {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rows: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cols: Option<i64>,
    }
    impl TerminalSize {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerExecResponse {
    #[serde(rename = "webSocketUri", default, skip_serializing_if = "Option::is_none")]
    pub web_socket_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ContainerExecResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<container_group::Properties>,
}
impl ContainerGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod container_group {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<String>,
        pub containers: Vec<Container>,
        #[serde(rename = "imageRegistryCredentials", default, skip_serializing_if = "Vec::is_empty")]
        pub image_registry_credentials: Vec<ImageRegistryCredential>,
        #[serde(rename = "restartPolicy", default, skip_serializing_if = "Option::is_none")]
        pub restart_policy: Option<properties::RestartPolicy>,
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
        pub ip_address: Option<IpAddress>,
        #[serde(rename = "osType")]
        pub os_type: properties::OsType,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub volumes: Vec<Volume>,
        #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
        pub instance_view: Option<properties::InstanceView>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub diagnostics: Option<ContainerGroupDiagnostics>,
        #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
        pub network_profile: Option<ContainerGroupNetworkProfile>,
    }
    impl Properties {
        pub fn new(containers: Vec<Container>, os_type: properties::OsType) -> Self {
            Self {
                provisioning_state: None,
                containers,
                image_registry_credentials: Vec::new(),
                restart_policy: None,
                ip_address: None,
                os_type,
                volumes: Vec::new(),
                instance_view: None,
                diagnostics: None,
                network_profile: None,
            }
        }
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum RestartPolicy {
            Always,
            OnFailure,
            Never,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum OsType {
            Windows,
            Linux,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct InstanceView {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub events: Vec<Event>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub state: Option<String>,
        }
        impl InstanceView {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerGroupDiagnostics {
    #[serde(rename = "logAnalytics", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics: Option<LogAnalytics>,
}
impl ContainerGroupDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerGroupListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ContainerGroup>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ContainerGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroupNetworkProfile {
    pub id: String,
}
impl ContainerGroupNetworkProfile {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerHttpGet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub port: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<container_http_get::Scheme>,
}
impl ContainerHttpGet {
    pub fn new(port: i32) -> Self {
        Self {
            path: None,
            port,
            scheme: None,
        }
    }
}
pub mod container_http_get {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scheme {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "https")]
        Https,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<container_port::Protocol>,
    pub port: i32,
}
impl ContainerPort {
    pub fn new(port: i32) -> Self {
        Self { protocol: None, port }
    }
}
pub mod container_port {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerProbe {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ContainerExec>,
    #[serde(rename = "httpGet", default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<ContainerHttpGet>,
    #[serde(rename = "initialDelaySeconds", default, skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    #[serde(rename = "periodSeconds", default, skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    #[serde(rename = "failureThreshold", default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[serde(rename = "successThreshold", default, skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    #[serde(rename = "timeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}
impl ContainerProbe {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerProperties {
    pub image: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ContainerPort>,
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentVariable>,
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<container_properties::InstanceView>,
    pub resources: ResourceRequirements,
    #[serde(rename = "volumeMounts", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMount>,
    #[serde(rename = "livenessProbe", default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<ContainerProbe>,
    #[serde(rename = "readinessProbe", default, skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<ContainerProbe>,
}
impl ContainerProperties {
    pub fn new(image: String, resources: ResourceRequirements) -> Self {
        Self {
            image,
            command: Vec::new(),
            ports: Vec::new(),
            environment_variables: Vec::new(),
            instance_view: None,
            resources,
            volume_mounts: Vec::new(),
            liveness_probe: None,
            readiness_probe: None,
        }
    }
}
pub mod container_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InstanceView {
        #[serde(rename = "restartCount", default, skip_serializing_if = "Option::is_none")]
        pub restart_count: Option<i64>,
        #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
        pub current_state: Option<ContainerState>,
        #[serde(rename = "previousState", default, skip_serializing_if = "Option::is_none")]
        pub previous_state: Option<ContainerState>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub events: Vec<Event>,
    }
    impl InstanceView {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmptyDirVolume {}
impl EmptyDirVolume {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "secureValue", default, skip_serializing_if = "Option::is_none")]
    pub secure_value: Option<String>,
}
impl EnvironmentVariable {
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: None,
            secure_value: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Event {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "firstTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
    #[serde(rename = "lastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Event {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitRepoVolume {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    pub repository: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}
impl GitRepoVolume {
    pub fn new(repository: String) -> Self {
        Self {
            directory: None,
            repository,
            revision: None,
        }
    }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAddress {
    pub ports: Vec<Port>,
    #[serde(rename = "type")]
    pub type_: ip_address::Type,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "dnsNameLabel", default, skip_serializing_if = "Option::is_none")]
    pub dns_name_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
}
impl IpAddress {
    pub fn new(ports: Vec<Port>, type_: ip_address::Type) -> Self {
        Self {
            ports,
            type_,
            ip: None,
            dns_name_label: None,
            fqdn: None,
        }
    }
}
pub mod ip_address {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Public,
        Private,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogAnalytics {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "workspaceKey")]
    pub workspace_key: String,
    #[serde(rename = "logType", default, skip_serializing_if = "Option::is_none")]
    pub log_type: Option<log_analytics::LogType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl LogAnalytics {
    pub fn new(workspace_id: String, workspace_key: String) -> Self {
        Self {
            workspace_id,
            workspace_key,
            log_type: None,
            metadata: None,
        }
    }
}
pub mod log_analytics {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LogType {
        ContainerInsights,
        ContainerInstanceLogs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Logs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl Logs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    pub name: String,
    pub display: operation::Display,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
}
impl Operation {
    pub fn new(name: String, display: operation::Display) -> Self {
        Self {
            name,
            display,
            origin: None,
        }
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        User,
        System,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Port {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<port::Protocol>,
    pub port: i32,
}
impl Port {
    pub fn new(port: i32) -> Self {
        Self { protocol: None, port }
    }
}
pub mod port {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
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
pub struct SecretVolume {}
impl SecretVolume {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<usage::Name>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Name {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
        pub localized_value: Option<String>,
    }
    impl Name {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
impl UsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub name: String,
    #[serde(rename = "azureFile", default, skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<AzureFileVolume>,
    #[serde(rename = "emptyDir", default, skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<EmptyDirVolume>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretVolume>,
    #[serde(rename = "gitRepo", default, skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<GitRepoVolume>,
}
impl Volume {
    pub fn new(name: String) -> Self {
        Self {
            name,
            azure_file: None,
            empty_dir: None,
            secret: None,
            git_repo: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeMount {
    pub name: String,
    #[serde(rename = "mountPath")]
    pub mount_path: String,
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
impl VolumeMount {
    pub fn new(name: String, mount_path: String) -> Self {
        Self {
            name,
            mount_path,
            read_only: None,
        }
    }
}
