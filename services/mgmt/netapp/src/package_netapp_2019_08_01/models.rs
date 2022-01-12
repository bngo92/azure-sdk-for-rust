#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "resourceIdDimensionNameOverride", default, skip_serializing_if = "Option::is_none")]
    pub resource_id_dimension_name_override: Option<String>,
}
impl MetricSpecification {
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
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
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
pub struct ResourceNameAvailability {
    #[serde(rename = "isAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<resource_name_availability::Reason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ResourceNameAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_name_availability {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNameAvailabilityRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: resource_name_availability_request::Type,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
}
impl ResourceNameAvailabilityRequest {
    pub fn new(name: String, type_: resource_name_availability_request::Type, resource_group: String) -> Self {
        Self {
            name,
            type_,
            resource_group,
        }
    }
}
pub mod resource_name_availability_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.NetApp/netAppAccounts")]
        MicrosoftNetAppNetAppAccounts,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools")]
        MicrosoftNetAppNetAppAccountsCapacityPools,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools/volumes")]
        MicrosoftNetAppNetAppAccountsCapacityPoolsVolumes,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools/volumes/snapshots")]
        MicrosoftNetAppNetAppAccountsCapacityPoolsVolumesSnapshots,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "activeDirectories", default, skip_serializing_if = "Vec::is_empty")]
    pub active_directories: Vec<ActiveDirectory>,
}
impl AccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectory {
    #[serde(rename = "activeDirectoryId", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "smbServerName", default, skip_serializing_if = "Option::is_none")]
    pub smb_server_name: Option<String>,
    #[serde(rename = "organizationalUnit", default, skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<String>,
}
impl ActiveDirectory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacityPool {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    pub properties: PoolProperties,
}
impl CapacityPool {
    pub fn new(location: String, properties: PoolProperties) -> Self {
        Self {
            location,
            id: None,
            name: None,
            type_: None,
            tags: None,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityPoolList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CapacityPool>,
}
impl CapacityPoolList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityPoolPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolPatchProperties>,
}
impl CapacityPoolPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportPolicyRule {
    #[serde(rename = "ruleIndex", default, skip_serializing_if = "Option::is_none")]
    pub rule_index: Option<i64>,
    #[serde(rename = "unixReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub unix_read_only: Option<bool>,
    #[serde(rename = "unixReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub unix_read_write: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cifs: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfsv3: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfsv41: Option<bool>,
    #[serde(rename = "allowedClients", default, skip_serializing_if = "Option::is_none")]
    pub allowed_clients: Option<String>,
}
impl ExportPolicyRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountTarget {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    pub properties: MountTargetProperties,
}
impl MountTarget {
    pub fn new(location: String, properties: MountTargetProperties) -> Self {
        Self {
            location,
            id: None,
            name: None,
            type_: None,
            tags: None,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MountTargetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MountTarget>,
}
impl MountTargetList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountTargetProperties {
    #[serde(rename = "mountTargetId", default, skip_serializing_if = "Option::is_none")]
    pub mount_target_id: Option<String>,
    #[serde(rename = "fileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[serde(rename = "startIp", default, skip_serializing_if = "Option::is_none")]
    pub start_ip: Option<String>,
    #[serde(rename = "endIp", default, skip_serializing_if = "Option::is_none")]
    pub end_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
    #[serde(rename = "smbServerFqdn", default, skip_serializing_if = "Option::is_none")]
    pub smb_server_fqdn: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl MountTargetProperties {
    pub fn new(file_system_id: String) -> Self {
        Self {
            mount_target_id: None,
            file_system_id,
            ip_address: None,
            subnet: None,
            start_ip: None,
            end_ip: None,
            gateway: None,
            netmask: None,
            smb_server_fqdn: None,
            provisioning_state: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetAppAccount {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
impl NetAppAccount {
    pub fn new(location: String) -> Self {
        Self {
            location,
            id: None,
            name: None,
            type_: None,
            tags: None,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetAppAccountList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetAppAccount>,
}
impl NetAppAccountList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetAppAccountPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
impl NetAppAccountPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolPatchProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "serviceLevel", default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<pool_patch_properties::ServiceLevel>,
}
impl PoolPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pool_patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceLevel {
        Standard,
        Premium,
        Ultra,
    }
    impl Default for ServiceLevel {
        fn default() -> Self {
            Self::Premium
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolProperties {
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    pub size: i64,
    #[serde(rename = "serviceLevel")]
    pub service_level: pool_properties::ServiceLevel,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl PoolProperties {
    pub fn new(size: i64, service_level: pool_properties::ServiceLevel) -> Self {
        Self {
            pool_id: None,
            size,
            service_level,
            provisioning_state: None,
        }
    }
}
pub mod pool_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceLevel {
        Standard,
        Premium,
        Ultra,
    }
    impl Default for ServiceLevel {
        fn default() -> Self {
            Self::Premium
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationObject {
    #[serde(rename = "replicationId", default, skip_serializing_if = "Option::is_none")]
    pub replication_id: Option<String>,
    #[serde(rename = "endpointType")]
    pub endpoint_type: String,
    #[serde(rename = "replicationSchedule")]
    pub replication_schedule: String,
    #[serde(rename = "remoteVolumeResourceId")]
    pub remote_volume_resource_id: String,
}
impl ReplicationObject {
    pub fn new(endpoint_type: String, replication_schedule: String, remote_volume_resource_id: String) -> Self {
        Self {
            replication_id: None,
            endpoint_type,
            replication_schedule,
            remote_volume_resource_id,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTags {}
impl ResourceTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotProperties>,
}
impl Snapshot {
    pub fn new(location: String) -> Self {
        Self {
            location,
            id: None,
            name: None,
            type_: None,
            tags: None,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
}
impl SnapshotPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotProperties {
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "fileSystemId", default, skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl SnapshotProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Snapshot>,
}
impl SnapshotsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    pub properties: VolumeProperties,
}
impl Volume {
    pub fn new(location: String, properties: VolumeProperties) -> Self {
        Self {
            location,
            id: None,
            name: None,
            type_: None,
            tags: None,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Volume>,
}
impl VolumeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumePatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumePatchProperties>,
}
impl VolumePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumePatchProperties {
    #[serde(rename = "serviceLevel", default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<volume_patch_properties::ServiceLevel>,
    #[serde(rename = "usageThreshold", default, skip_serializing_if = "Option::is_none")]
    pub usage_threshold: Option<i64>,
    #[serde(rename = "exportPolicy", default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<volume_patch_properties::ExportPolicy>,
}
impl VolumePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod volume_patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceLevel {
        Standard,
        Premium,
        Ultra,
    }
    impl Default for ServiceLevel {
        fn default() -> Self {
            Self::Premium
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ExportPolicy {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub rules: Vec<ExportPolicyRule>,
    }
    impl ExportPolicy {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProperties {
    #[serde(rename = "fileSystemId", default, skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "creationToken")]
    pub creation_token: String,
    #[serde(rename = "serviceLevel", default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<volume_properties::ServiceLevel>,
    #[serde(rename = "usageThreshold")]
    pub usage_threshold: i64,
    #[serde(rename = "usedBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_bytes: Option<i64>,
    #[serde(rename = "exportPolicy", default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<volume_properties::ExportPolicy>,
    #[serde(rename = "protocolTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_types: Vec<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "baremetalTenantId", default, skip_serializing_if = "Option::is_none")]
    pub baremetal_tenant_id: Option<String>,
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[serde(rename = "mountTargets", default, skip_serializing_if = "Vec::is_empty")]
    pub mount_targets: Vec<MountTargetProperties>,
    #[serde(rename = "volumeType", default, skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
    #[serde(rename = "dataProtection", default, skip_serializing_if = "Option::is_none")]
    pub data_protection: Option<volume_properties::DataProtection>,
}
impl VolumeProperties {
    pub fn new(creation_token: String, usage_threshold: i64, subnet_id: String) -> Self {
        Self {
            file_system_id: None,
            creation_token,
            service_level: None,
            usage_threshold,
            used_bytes: None,
            export_policy: None,
            protocol_types: Vec::new(),
            provisioning_state: None,
            snapshot_id: None,
            baremetal_tenant_id: None,
            subnet_id,
            mount_targets: Vec::new(),
            volume_type: None,
            data_protection: None,
        }
    }
}
pub mod volume_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceLevel {
        Standard,
        Premium,
        Ultra,
    }
    impl Default for ServiceLevel {
        fn default() -> Self {
            Self::Premium
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ExportPolicy {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub rules: Vec<ExportPolicyRule>,
    }
    impl ExportPolicy {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DataProtection {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub replication: Option<ReplicationObject>,
    }
    impl DataProtection {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
