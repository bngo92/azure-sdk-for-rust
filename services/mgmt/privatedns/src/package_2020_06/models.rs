#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ARecord {
    #[serde(rename = "ipv4Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
}
impl ARecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AaaaRecord {
    #[serde(rename = "ipv6Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
}
impl AaaaRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CnameRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
}
impl CnameRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MxRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preference: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
}
impl MxRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateZone {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateZoneProperties>,
}
impl PrivateZone {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateZoneListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateZone>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PrivateZoneListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateZoneProperties {
    #[serde(rename = "maxNumberOfRecordSets", default, skip_serializing_if = "Option::is_none")]
    pub max_number_of_record_sets: Option<i64>,
    #[serde(rename = "numberOfRecordSets", default, skip_serializing_if = "Option::is_none")]
    pub number_of_record_sets: Option<i64>,
    #[serde(rename = "maxNumberOfVirtualNetworkLinks", default, skip_serializing_if = "Option::is_none")]
    pub max_number_of_virtual_network_links: Option<i64>,
    #[serde(rename = "numberOfVirtualNetworkLinks", default, skip_serializing_if = "Option::is_none")]
    pub number_of_virtual_network_links: Option<i64>,
    #[serde(
        rename = "maxNumberOfVirtualNetworkLinksWithRegistration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_number_of_virtual_network_links_with_registration: Option<i64>,
    #[serde(
        rename = "numberOfVirtualNetworkLinksWithRegistration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_virtual_network_links_with_registration: Option<i64>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_zone_properties::ProvisioningState>,
    #[serde(rename = "internalId", default, skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,
}
impl PrivateZoneProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_zone_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
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
pub struct PtrRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ptrdname: Option<String>,
}
impl PtrRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecordSet {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecordSetProperties>,
}
impl RecordSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecordSetListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecordSet>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RecordSetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecordSetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "isAutoRegistered", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_registered: Option<bool>,
    #[serde(rename = "aRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub a_records: Vec<ARecord>,
    #[serde(rename = "aaaaRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub aaaa_records: Vec<AaaaRecord>,
    #[serde(rename = "cnameRecord", default, skip_serializing_if = "Option::is_none")]
    pub cname_record: Option<CnameRecord>,
    #[serde(rename = "mxRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub mx_records: Vec<MxRecord>,
    #[serde(rename = "ptrRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub ptr_records: Vec<PtrRecord>,
    #[serde(rename = "soaRecord", default, skip_serializing_if = "Option::is_none")]
    pub soa_record: Option<SoaRecord>,
    #[serde(rename = "srvRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub srv_records: Vec<SrvRecord>,
    #[serde(rename = "txtRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub txt_records: Vec<TxtRecord>,
}
impl RecordSetProperties {
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
pub struct SoaRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<i64>,
    #[serde(rename = "refreshTime", default, skip_serializing_if = "Option::is_none")]
    pub refresh_time: Option<i64>,
    #[serde(rename = "retryTime", default, skip_serializing_if = "Option::is_none")]
    pub retry_time: Option<i64>,
    #[serde(rename = "expireTime", default, skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[serde(rename = "minimumTtl", default, skip_serializing_if = "Option::is_none")]
    pub minimum_ttl: Option<i64>,
}
impl SoaRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SrvRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl SrvRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TxtRecord {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
impl TxtRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLink {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkLinkProperties>,
}
impl VirtualNetworkLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLinkListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetworkLink>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl VirtualNetworkLinkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLinkProperties {
    #[serde(rename = "virtualNetwork", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network: Option<SubResource>,
    #[serde(rename = "registrationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub registration_enabled: Option<bool>,
    #[serde(rename = "virtualNetworkLinkState", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_link_state: Option<virtual_network_link_properties::VirtualNetworkLinkState>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<virtual_network_link_properties::ProvisioningState>,
}
impl VirtualNetworkLinkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_network_link_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VirtualNetworkLinkState {
        InProgress,
        Completed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
}
