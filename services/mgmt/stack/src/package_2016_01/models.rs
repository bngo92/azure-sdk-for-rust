#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivationKeyResult {
    #[serde(rename = "activationKey", default, skip_serializing_if = "Option::is_none")]
    pub activation_key: Option<String>,
}
impl ActivationKeyResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Compatibility {
    #[serde(rename = "isCompatible", default, skip_serializing_if = "Option::is_none")]
    pub is_compatible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<CompatibilityIssue>,
}
impl Compatibility {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CompatibilityIssue {
    HigherDeviceVersionRequired,
    LowerDeviceVersionRequired,
    CapacityBillingModelRequired,
    PayAsYouGoBillingModelRequired,
    DevelopmentBillingModelRequired,
    #[serde(rename = "AzureADIdentitySystemRequired")]
    AzureAdIdentitySystemRequired,
    #[serde(rename = "ADFSIdentitySystemRequired")]
    AdfsIdentitySystemRequired,
    ConnectionToInternetRequired,
    ConnectionToAzureRequired,
    DisconnectedEnvironmentRequired,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputeRole {
    None,
    IaaS,
    PaaS,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataDiskImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[serde(rename = "sourceBlobSasUri", default, skip_serializing_if = "Option::is_none")]
    pub source_blob_sas_uri: Option<String>,
}
impl DataDiskImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceConfiguration {
    #[serde(rename = "deviceVersion", default, skip_serializing_if = "Option::is_none")]
    pub device_version: Option<String>,
    #[serde(rename = "identitySystem", default, skip_serializing_if = "Option::is_none")]
    pub identity_system: Option<device_configuration::IdentitySystem>,
}
impl DeviceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod device_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IdentitySystem {
        #[serde(rename = "AzureAD")]
        AzureAd,
        #[serde(rename = "ADFS")]
        Adfs,
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
pub struct ExtendedProduct {
    #[serde(rename = "galleryPackageBlobSasUri", default, skip_serializing_if = "Option::is_none")]
    pub gallery_package_blob_sas_uri: Option<String>,
    #[serde(rename = "productKind", default, skip_serializing_if = "Option::is_none")]
    pub product_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExtendedProductProperties>,
}
impl ExtendedProduct {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedProductProperties {
    #[serde(flatten)]
    pub virtual_machine_extension_product_properties: VirtualMachineExtensionProductProperties,
    #[serde(flatten)]
    pub virtual_machine_product_properties: VirtualMachineProductProperties,
}
impl ExtendedProductProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IconUris {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wide: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hero: Option<String>,
}
impl IconUris {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplaceProductLogUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl MarketplaceProductLogUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperatingSystem {
    None,
    Windows,
    Linux,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
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
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionType {
        Internal,
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
pub struct OsDiskImage {
    #[serde(rename = "operatingSystem", default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<OperatingSystem>,
    #[serde(rename = "sourceBlobSasUri", default, skip_serializing_if = "Option::is_none")]
    pub source_blob_sas_uri: Option<String>,
}
impl OsDiskImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Product {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductNestedProperties>,
}
impl Product {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductLink {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl ProductLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Product>,
}
impl ProductList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductLog {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "registrationName", default, skip_serializing_if = "Option::is_none")]
    pub registration_name: Option<String>,
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl ProductLog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductNestedProperties {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<String>,
    #[serde(rename = "publisherIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub publisher_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(rename = "offerVersion", default, skip_serializing_if = "Option::is_none")]
    pub offer_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "billingPartNumber", default, skip_serializing_if = "Option::is_none")]
    pub billing_part_number: Option<String>,
    #[serde(rename = "vmExtensionType", default, skip_serializing_if = "Option::is_none")]
    pub vm_extension_type: Option<String>,
    #[serde(rename = "galleryItemIdentity", default, skip_serializing_if = "Option::is_none")]
    pub gallery_item_identity: Option<String>,
    #[serde(rename = "iconUris", default, skip_serializing_if = "Option::is_none")]
    pub icon_uris: Option<IconUris>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<ProductLink>,
    #[serde(rename = "legalTerms", default, skip_serializing_if = "Option::is_none")]
    pub legal_terms: Option<String>,
    #[serde(rename = "privacyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<String>,
    #[serde(rename = "payloadLength", default, skip_serializing_if = "Option::is_none")]
    pub payload_length: Option<i64>,
    #[serde(rename = "productKind", default, skip_serializing_if = "Option::is_none")]
    pub product_kind: Option<String>,
    #[serde(rename = "productProperties", default, skip_serializing_if = "Option::is_none")]
    pub product_properties: Option<ProductProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<Compatibility>,
}
impl ProductNestedProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ProductProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Registration {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistrationProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Registration {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            etag: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Registration>,
}
impl RegistrationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationParameter {
    pub properties: RegistrationParameterProperties,
    pub location: registration_parameter::Location,
}
impl RegistrationParameter {
    pub fn new(properties: RegistrationParameterProperties, location: registration_parameter::Location) -> Self {
        Self { properties, location }
    }
}
pub mod registration_parameter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Location {
        #[serde(rename = "global")]
        Global,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationParameterProperties {
    #[serde(rename = "registrationToken")]
    pub registration_token: String,
}
impl RegistrationParameterProperties {
    pub fn new(registration_token: String) -> Self {
        Self { registration_token }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationProperties {
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "cloudId", default, skip_serializing_if = "Option::is_none")]
    pub cloud_id: Option<String>,
    #[serde(rename = "billingModel", default, skip_serializing_if = "Option::is_none")]
    pub billing_model: Option<String>,
}
impl RegistrationProperties {
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
pub struct Uri {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl Uri {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineExtensionProductProperties {
    #[serde(rename = "computeRole", default, skip_serializing_if = "Option::is_none")]
    pub compute_role: Option<ComputeRole>,
    #[serde(rename = "isSystemExtension", default, skip_serializing_if = "Option::is_none")]
    pub is_system_extension: Option<bool>,
    #[serde(rename = "sourceBlob", default, skip_serializing_if = "Option::is_none")]
    pub source_blob: Option<Uri>,
    #[serde(rename = "supportMultipleExtensions", default, skip_serializing_if = "Option::is_none")]
    pub support_multiple_extensions: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "vmOsType", default, skip_serializing_if = "Option::is_none")]
    pub vm_os_type: Option<OperatingSystem>,
    #[serde(rename = "vmScaleSetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub vm_scale_set_enabled: Option<bool>,
}
impl VirtualMachineExtensionProductProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineProductProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "osDiskImage", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_image: Option<OsDiskImage>,
    #[serde(rename = "dataDiskImages", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disk_images: Vec<DataDiskImage>,
}
impl VirtualMachineProductProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
