#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalErrorInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl AdditionalErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalInventoryDetails {
    #[serde(rename = "additionalData", default, skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
}
impl AdditionalInventoryDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalOrderItemDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StageDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionDetails>,
}
impl AdditionalOrderItemDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingDetails {
    #[serde(rename = "billingType", default, skip_serializing_if = "Option::is_none")]
    pub billing_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl BillingDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudError>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<AdditionalErrorInfo>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationData {
    #[serde(rename = "familyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub family_identifier: Option<String>,
    #[serde(rename = "productLineIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub product_line_identifier: Option<String>,
    #[serde(rename = "productIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub product_identifier: Option<String>,
    #[serde(rename = "configurationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub configuration_identifier: Option<String>,
    #[serde(rename = "configurationIdentifierOnDevice", default, skip_serializing_if = "Option::is_none")]
    pub configuration_identifier_on_device: Option<String>,
}
impl ConfigurationData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationDetails {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specifications: Vec<SpecificationDetails>,
}
impl ConfigurationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationOnDevice {
    #[serde(rename = "configurationIdentifier")]
    pub configuration_identifier: String,
}
impl ConfigurationOnDevice {
    pub fn new(configuration_identifier: String) -> Self {
        Self { configuration_identifier }
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
pub struct InventoryAdditionalDetails {
    #[serde(rename = "orderItem", default, skip_serializing_if = "Option::is_none")]
    pub order_item: Option<AdditionalOrderItemDetails>,
    #[serde(rename = "inventoryMetadata", default, skip_serializing_if = "Option::is_none")]
    pub inventory_metadata: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory: Option<AdditionalInventoryDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing: Option<BillingDetails>,
    #[serde(rename = "inventorySecrets", default, skip_serializing_if = "Option::is_none")]
    pub inventory_secrets: Option<serde_json::Value>,
}
impl InventoryAdditionalDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InventoryData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "registrationAllowed", default, skip_serializing_if = "Option::is_none")]
    pub registration_allowed: Option<bool>,
}
impl InventoryData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InventoryProperties {
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "orderItem", default, skip_serializing_if = "Option::is_none")]
    pub order_item: Option<OrderItemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationData>,
    #[serde(rename = "managementResource", default, skip_serializing_if = "Option::is_none")]
    pub management_resource: Option<ManagementResourceData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory: Option<InventoryData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<InventoryAdditionalDetails>,
}
impl InventoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManageInventoryMetadataRequest {
    #[serde(rename = "inventoryMetadata")]
    pub inventory_metadata: String,
    #[serde(rename = "configurationOnDevice", default, skip_serializing_if = "Option::is_none")]
    pub configuration_on_device: Option<ConfigurationOnDevice>,
}
impl ManageInventoryMetadataRequest {
    pub fn new(inventory_metadata: String) -> Self {
        Self {
            inventory_metadata,
            configuration_on_device: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManageLinkRequest {
    #[serde(rename = "managementResourceArmId")]
    pub management_resource_arm_id: String,
    pub operation: manage_link_request::Operation,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
impl ManageLinkRequest {
    pub fn new(management_resource_arm_id: String, operation: manage_link_request::Operation, tenant_id: String) -> Self {
        Self {
            management_resource_arm_id,
            operation,
            tenant_id,
        }
    }
}
pub mod manage_link_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        Link,
        Unlink,
        Relink,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementResourceData {
    #[serde(rename = "armId", default, skip_serializing_if = "Option::is_none")]
    pub arm_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ManagementResourceData {
    pub fn new() -> Self {
        Self::default()
    }
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
pub struct OrderItemData {
    #[serde(rename = "armId", default, skip_serializing_if = "Option::is_none")]
    pub arm_id: Option<String>,
    #[serde(rename = "orderItemType", default, skip_serializing_if = "Option::is_none")]
    pub order_item_type: Option<order_item_data::OrderItemType>,
}
impl OrderItemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod order_item_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OrderItemType {
        Purchase,
        Rental,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerInventory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InventoryProperties>,
}
impl PartnerInventory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerInventoryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PartnerInventory>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PartnerInventoryList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchInventoriesRequest {
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "familyIdentifier")]
    pub family_identifier: String,
}
impl SearchInventoriesRequest {
    pub fn new(serial_number: String, family_identifier: String) -> Self {
        Self {
            serial_number,
            family_identifier,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpecificationDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SpecificationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StageDetails {
    #[serde(rename = "stageStatus", default, skip_serializing_if = "Option::is_none")]
    pub stage_status: Option<stage_details::StageStatus>,
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<stage_details::StageName>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}
impl StageDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod stage_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageStatus {
        None,
        InProgress,
        Succeeded,
        Failed,
        Cancelled,
        Cancelling,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageName {
        DeviceOrdered,
        DevicePrepared,
        PickedUp,
        #[serde(rename = "AtAzureDC")]
        AtAzureDc,
        DataCopy,
        Completed,
        CompletedWithErrors,
        Cancelled,
        Aborted,
        CompletedWithWarnings,
        #[serde(rename = "ReadyToDispatchFromAzureDC")]
        ReadyToDispatchFromAzureDc,
        #[serde(rename = "ReadyToReceiveAtAzureDC")]
        ReadyToReceiveAtAzureDc,
        Placed,
        InReview,
        Confirmed,
        ReadyForDispatch,
        Shipped,
        Delivered,
        InUse,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "quotaId", default, skip_serializing_if = "Option::is_none")]
    pub quota_id: Option<String>,
}
impl SubscriptionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
