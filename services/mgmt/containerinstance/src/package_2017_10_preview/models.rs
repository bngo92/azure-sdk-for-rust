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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<container_group::Properties>,
}
impl ContainerGroup {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
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
    pub value: String,
}
impl EnvironmentVariable {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
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
}
impl IpAddress {
    pub fn new(ports: Vec<Port>, type_: ip_address::Type) -> Self {
        Self { ports, type_, ip: None }
    }
}
pub mod ip_address {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Public,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub name: String,
    #[serde(rename = "azureFile", default, skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<AzureFileVolume>,
    #[serde(rename = "emptyDir", default, skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<EmptyDirVolume>,
}
impl Volume {
    pub fn new(name: String) -> Self {
        Self {
            name,
            azure_file: None,
            empty_dir: None,
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
