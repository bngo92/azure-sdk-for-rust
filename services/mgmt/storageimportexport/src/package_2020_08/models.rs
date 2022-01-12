#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryPackageInformation {
    #[serde(rename = "carrierName")]
    pub carrier_name: String,
    #[serde(rename = "trackingNumber")]
    pub tracking_number: String,
    #[serde(rename = "driveCount", default, skip_serializing_if = "Option::is_none")]
    pub drive_count: Option<i64>,
    #[serde(rename = "shipDate", default, skip_serializing_if = "Option::is_none")]
    pub ship_date: Option<String>,
}
impl DeliveryPackageInformation {
    pub fn new(carrier_name: String, tracking_number: String) -> Self {
        Self {
            carrier_name,
            tracking_number,
            drive_count: None,
            ship_date: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DriveBitLockerKey {
    #[serde(rename = "bitLockerKey", default, skip_serializing_if = "Option::is_none")]
    pub bit_locker_key: Option<String>,
    #[serde(rename = "driveId", default, skip_serializing_if = "Option::is_none")]
    pub drive_id: Option<String>,
}
impl DriveBitLockerKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DriveStatus {
    #[serde(rename = "driveId", default, skip_serializing_if = "Option::is_none")]
    pub drive_id: Option<String>,
    #[serde(rename = "bitLockerKey", default, skip_serializing_if = "Option::is_none")]
    pub bit_locker_key: Option<String>,
    #[serde(rename = "manifestFile", default, skip_serializing_if = "Option::is_none")]
    pub manifest_file: Option<String>,
    #[serde(rename = "manifestHash", default, skip_serializing_if = "Option::is_none")]
    pub manifest_hash: Option<String>,
    #[serde(rename = "driveHeaderHash", default, skip_serializing_if = "Option::is_none")]
    pub drive_header_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<drive_status::State>,
    #[serde(rename = "copyStatus", default, skip_serializing_if = "Option::is_none")]
    pub copy_status: Option<String>,
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[serde(rename = "verboseLogUri", default, skip_serializing_if = "Option::is_none")]
    pub verbose_log_uri: Option<String>,
    #[serde(rename = "errorLogUri", default, skip_serializing_if = "Option::is_none")]
    pub error_log_uri: Option<String>,
    #[serde(rename = "manifestUri", default, skip_serializing_if = "Option::is_none")]
    pub manifest_uri: Option<String>,
    #[serde(rename = "bytesSucceeded", default, skip_serializing_if = "Option::is_none")]
    pub bytes_succeeded: Option<i64>,
}
impl DriveStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod drive_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Specified,
        Received,
        NeverReceived,
        Transferring,
        Completed,
        CompletedMoreInfo,
        ShippedBack,
    }
    impl Default for State {
        fn default() -> Self {
            Self::Specified
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionKeyDetails {
    #[serde(rename = "kekType", default, skip_serializing_if = "Option::is_none")]
    pub kek_type: Option<encryption_key_details::KekType>,
    #[serde(rename = "kekUrl", default, skip_serializing_if = "Option::is_none")]
    pub kek_url: Option<String>,
    #[serde(rename = "kekVaultResourceID", default, skip_serializing_if = "Option::is_none")]
    pub kek_vault_resource_id: Option<String>,
}
impl EncryptionKeyDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_key_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KekType {
        MicrosoftManaged,
        CustomerManaged,
    }
    impl Default for KekType {
        fn default() -> Self {
            Self::MicrosoftManaged
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub innererror: Option<serde_json::Value>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Export {
    #[serde(rename = "blobList", default, skip_serializing_if = "Option::is_none")]
    pub blob_list: Option<export::BlobList>,
    #[serde(rename = "blobListBlobPath", default, skip_serializing_if = "Option::is_none")]
    pub blob_list_blob_path: Option<String>,
}
impl Export {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod export {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct BlobList {
        #[serde(rename = "blobPath", default, skip_serializing_if = "Vec::is_empty")]
        pub blob_path: Vec<String>,
        #[serde(rename = "blobPathPrefix", default, skip_serializing_if = "Vec::is_empty")]
        pub blob_path_prefix: Vec<String>,
    }
    impl BlobList {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetBitLockerKeysResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DriveBitLockerKey>,
}
impl GetBitLockerKeysResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityDetails {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity_details::Type>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl IdentityDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
    }
    impl Default for Type {
        fn default() -> Self {
            Self::None
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDetails {
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[serde(rename = "jobType", default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(rename = "returnAddress", default, skip_serializing_if = "Option::is_none")]
    pub return_address: Option<ReturnAddress>,
    #[serde(rename = "returnShipping", default, skip_serializing_if = "Option::is_none")]
    pub return_shipping: Option<ReturnShipping>,
    #[serde(rename = "shippingInformation", default, skip_serializing_if = "Option::is_none")]
    pub shipping_information: Option<ShippingInformation>,
    #[serde(rename = "deliveryPackage", default, skip_serializing_if = "Option::is_none")]
    pub delivery_package: Option<DeliveryPackageInformation>,
    #[serde(rename = "returnPackage", default, skip_serializing_if = "Option::is_none")]
    pub return_package: Option<PackageInformation>,
    #[serde(rename = "diagnosticsPath", default, skip_serializing_if = "Option::is_none")]
    pub diagnostics_path: Option<String>,
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(rename = "backupDriveManifest", default, skip_serializing_if = "Option::is_none")]
    pub backup_drive_manifest: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "cancelRequested", default, skip_serializing_if = "Option::is_none")]
    pub cancel_requested: Option<bool>,
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[serde(rename = "incompleteBlobListUri", default, skip_serializing_if = "Option::is_none")]
    pub incomplete_blob_list_uri: Option<String>,
    #[serde(rename = "driveList", default, skip_serializing_if = "Vec::is_empty")]
    pub drive_list: Vec<DriveStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export: Option<Export>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKeyDetails>,
}
impl JobDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobResponse {
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityDetails>,
}
impl JobResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListJobsResponse {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobResponse>,
}
impl ListJobsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListOperationsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl ListOperationsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Location {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<location::Properties>,
}
impl Location {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod location {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "recipientName", default, skip_serializing_if = "Option::is_none")]
        pub recipient_name: Option<String>,
        #[serde(rename = "streetAddress1", default, skip_serializing_if = "Option::is_none")]
        pub street_address1: Option<String>,
        #[serde(rename = "streetAddress2", default, skip_serializing_if = "Option::is_none")]
        pub street_address2: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(rename = "stateOrProvince", default, skip_serializing_if = "Option::is_none")]
        pub state_or_province: Option<String>,
        #[serde(rename = "postalCode", default, skip_serializing_if = "Option::is_none")]
        pub postal_code: Option<String>,
        #[serde(rename = "countryOrRegion", default, skip_serializing_if = "Option::is_none")]
        pub country_or_region: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(rename = "additionalShippingInformation", default, skip_serializing_if = "Option::is_none")]
        pub additional_shipping_information: Option<String>,
        #[serde(rename = "supportedCarriers", default, skip_serializing_if = "Vec::is_empty")]
        pub supported_carriers: Vec<String>,
        #[serde(rename = "alternateLocations", default, skip_serializing_if = "Vec::is_empty")]
        pub alternate_locations: Vec<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Location>,
}
impl LocationsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    pub name: String,
    pub display: operation::Display,
}
impl Operation {
    pub fn new(name: String, display: operation::Display) -> Self {
        Self { name, display }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageInformation {
    #[serde(rename = "carrierName")]
    pub carrier_name: String,
    #[serde(rename = "trackingNumber")]
    pub tracking_number: String,
    #[serde(rename = "driveCount")]
    pub drive_count: i64,
    #[serde(rename = "shipDate")]
    pub ship_date: String,
}
impl PackageInformation {
    pub fn new(carrier_name: String, tracking_number: String, drive_count: i64, ship_date: String) -> Self {
        Self {
            carrier_name,
            tracking_number,
            drive_count,
            ship_date,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PutJobParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobDetails>,
}
impl PutJobParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnAddress {
    #[serde(rename = "recipientName")]
    pub recipient_name: String,
    #[serde(rename = "streetAddress1")]
    pub street_address1: String,
    #[serde(rename = "streetAddress2", default, skip_serializing_if = "Option::is_none")]
    pub street_address2: Option<String>,
    pub city: String,
    #[serde(rename = "stateOrProvince", default, skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    #[serde(rename = "countryOrRegion")]
    pub country_or_region: String,
    pub phone: String,
    pub email: String,
}
impl ReturnAddress {
    pub fn new(
        recipient_name: String,
        street_address1: String,
        city: String,
        postal_code: String,
        country_or_region: String,
        phone: String,
        email: String,
    ) -> Self {
        Self {
            recipient_name,
            street_address1,
            street_address2: None,
            city,
            state_or_province: None,
            postal_code,
            country_or_region,
            phone,
            email,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnShipping {
    #[serde(rename = "carrierName")]
    pub carrier_name: String,
    #[serde(rename = "carrierAccountNumber")]
    pub carrier_account_number: String,
}
impl ReturnShipping {
    pub fn new(carrier_name: String, carrier_account_number: String) -> Self {
        Self {
            carrier_name,
            carrier_account_number,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShippingInformation {
    #[serde(rename = "recipientName", default, skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,
    #[serde(rename = "streetAddress1", default, skip_serializing_if = "Option::is_none")]
    pub street_address1: Option<String>,
    #[serde(rename = "streetAddress2", default, skip_serializing_if = "Option::is_none")]
    pub street_address2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "stateOrProvince", default, skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    #[serde(rename = "postalCode", default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "countryOrRegion", default, skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "additionalInformation", default, skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<String>,
}
impl ShippingInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateJobParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<update_job_parameters::Properties>,
}
impl UpdateJobParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_job_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "cancelRequested", default, skip_serializing_if = "Option::is_none")]
        pub cancel_requested: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(rename = "returnAddress", default, skip_serializing_if = "Option::is_none")]
        pub return_address: Option<ReturnAddress>,
        #[serde(rename = "returnShipping", default, skip_serializing_if = "Option::is_none")]
        pub return_shipping: Option<ReturnShipping>,
        #[serde(rename = "deliveryPackage", default, skip_serializing_if = "Option::is_none")]
        pub delivery_package: Option<DeliveryPackageInformation>,
        #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
        pub log_level: Option<String>,
        #[serde(rename = "backupDriveManifest", default, skip_serializing_if = "Option::is_none")]
        pub backup_drive_manifest: Option<bool>,
        #[serde(rename = "driveList", default, skip_serializing_if = "Vec::is_empty")]
        pub drive_list: Vec<DriveStatus>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
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
