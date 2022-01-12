#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsyncOperationStatus {
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<async_operation_status::Status>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<B2cTenantResourceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<async_operation_status::Error>,
}
impl AsyncOperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod async_operation_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Succeeded,
        Pending,
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct B2cResourceSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<b2c_resource_sku::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<b2c_resource_sku::Tier>,
}
impl B2cResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod b2c_resource_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Standard,
        PremiumP1,
        PremiumP2,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        A0,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct B2cTenantResource {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<b2c_tenant_resource::Type>,
    pub sku: B2cResourceSku,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<B2cTenantResourceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl B2cTenantResource {
    pub fn new(sku: B2cResourceSku, location: String) -> Self {
        Self {
            type_: None,
            sku,
            properties: None,
            id: None,
            name: None,
            location,
            tags: None,
        }
    }
}
pub mod b2c_tenant_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.AzureActiveDirectory/b2cDirectories")]
        MicrosoftAzureActiveDirectoryB2cDirectories,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct B2cTenantResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<B2cTenantResource>,
}
impl B2cTenantResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct B2cTenantResourceProperties {
    #[serde(rename = "billingConfig", default, skip_serializing_if = "Option::is_none")]
    pub billing_config: Option<b2c_tenant_resource_properties::BillingConfig>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl B2cTenantResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod b2c_tenant_resource_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct BillingConfig {
        #[serde(rename = "billingType", default, skip_serializing_if = "Option::is_none")]
        pub billing_type: Option<billing_config::BillingType>,
        #[serde(rename = "effectiveStartDateUtc", default, skip_serializing_if = "Option::is_none")]
        pub effective_start_date_utc: Option<String>,
    }
    impl BillingConfig {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod billing_config {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum BillingType {
            #[serde(rename = "MAU")]
            Mau,
            Auths,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct B2cTenantUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<B2cResourceSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<B2cTenantResourceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl B2cTenantUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityRequestBody {
    pub name: String,
    #[serde(rename = "countryCode")]
    pub country_code: CountryCode,
}
impl CheckNameAvailabilityRequestBody {
    pub fn new(name: String, country_code: CountryCode) -> Self {
        Self { name, country_code }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type CountryCode = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateTenantProperties {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "countryCode", default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<CountryCode>,
}
impl CreateTenantProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTenantRequestBody {
    pub location: String,
    pub properties: create_tenant_request_body::Properties,
    pub sku: B2cResourceSku,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CreateTenantRequestBody {
    pub fn new(location: String, properties: create_tenant_request_body::Properties, sku: B2cResourceSku) -> Self {
        Self {
            location,
            properties,
            sku,
            tags: None,
        }
    }
}
pub mod create_tenant_request_body {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "createTenantProperties", default, skip_serializing_if = "Option::is_none")]
        pub create_tenant_properties: Option<CreateTenantProperties>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
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
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NameAvailabilityReason {
    AlreadyExists,
    Invalid,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailabilityResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<NameAvailabilityReason>,
}
impl NameAvailabilityResponse {
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
