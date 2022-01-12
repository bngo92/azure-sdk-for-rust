#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureAsyncOperationResults {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupInfoProperties>,
}
impl AzureAsyncOperationResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<check_name_availability_request::Type>,
}
impl CheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Management/managementGroups")]
        MicrosoftManagementManagementGroups,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateManagementGroupChildInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagementGroupChildType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<CreateManagementGroupChildInfo>,
}
impl CreateManagementGroupChildInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateManagementGroupDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
    #[serde(rename = "updatedTime", default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<CreateParentGroupInfo>,
}
impl CreateManagementGroupDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateManagementGroupProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<CreateManagementGroupDetails>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<CreateManagementGroupChildInfo>,
}
impl CreateManagementGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateManagementGroupRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CreateManagementGroupProperties>,
}
impl CreateManagementGroupRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateOrUpdateSettingsProperties {
    #[serde(rename = "requireAuthorizationForGroupCreation", default, skip_serializing_if = "Option::is_none")]
    pub require_authorization_for_group_creation: Option<bool>,
    #[serde(rename = "defaultManagementGroup", default, skip_serializing_if = "Option::is_none")]
    pub default_management_group: Option<String>,
}
impl CreateOrUpdateSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateOrUpdateSettingsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CreateOrUpdateSettingsProperties>,
}
impl CreateOrUpdateSettingsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateParentGroupInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl CreateParentGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DescendantInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DescendantInfoProperties>,
}
impl DescendantInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DescendantInfoProperties {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<DescendantParentGroupInfo>,
}
impl DescendantInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DescendantListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DescendantInfo>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DescendantListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DescendantParentGroupInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl DescendantParentGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityHierarchyItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EntityHierarchyItemProperties>,
}
impl EntityHierarchyItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityHierarchyItemProperties {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<EntityHierarchyItem>,
}
impl EntityHierarchyItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EntityInfoProperties>,
}
impl EntityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityInfoProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<EntityParentGroupInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(rename = "inheritedPermissions", default, skip_serializing_if = "Option::is_none")]
    pub inherited_permissions: Option<Permissions>,
    #[serde(rename = "numberOfDescendants", default, skip_serializing_if = "Option::is_none")]
    pub number_of_descendants: Option<i64>,
    #[serde(rename = "numberOfChildren", default, skip_serializing_if = "Option::is_none")]
    pub number_of_children: Option<i64>,
    #[serde(rename = "numberOfChildGroups", default, skip_serializing_if = "Option::is_none")]
    pub number_of_child_groups: Option<i64>,
    #[serde(rename = "parentDisplayNameChain", default, skip_serializing_if = "Vec::is_empty")]
    pub parent_display_name_chain: Vec<String>,
    #[serde(rename = "parentNameChain", default, skip_serializing_if = "Vec::is_empty")]
    pub parent_name_chain: Vec<String>,
}
impl EntityInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EntityInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl EntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityParentGroupInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl EntityParentGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchySettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HierarchySettingsProperties>,
}
impl HierarchySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchySettingsInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HierarchySettingsProperties>,
}
impl HierarchySettingsInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchySettingsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HierarchySettingsInfo>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl HierarchySettingsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchySettingsProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "requireAuthorizationForGroupCreation", default, skip_serializing_if = "Option::is_none")]
    pub require_authorization_for_group_creation: Option<bool>,
    #[serde(rename = "defaultManagementGroup", default, skip_serializing_if = "Option::is_none")]
    pub default_management_group: Option<String>,
}
impl HierarchySettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListSubscriptionUnderManagementGroup {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubscriptionUnderManagementGroup>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ListSubscriptionUnderManagementGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupProperties>,
}
impl ManagementGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupChildInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagementGroupChildType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<ManagementGroupChildInfo>,
}
impl ManagementGroupChildInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ManagementGroupChildType {
    #[serde(rename = "Microsoft.Management/managementGroups")]
    MicrosoftManagementManagementGroups,
    #[serde(rename = "/subscriptions")]
    Subscriptions,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
    #[serde(rename = "updatedTime", default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<ParentGroupInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path: Vec<ManagementGroupPathElement>,
    #[serde(rename = "managementGroupAncestors", default, skip_serializing_if = "Vec::is_empty")]
    pub management_group_ancestors: Vec<String>,
    #[serde(rename = "managementGroupAncestorsChain", default, skip_serializing_if = "Vec::is_empty")]
    pub management_group_ancestors_chain: Vec<ManagementGroupPathElement>,
}
impl ManagementGroupDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupInfoProperties>,
}
impl ManagementGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupInfoProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ManagementGroupInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagementGroupInfo>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ManagementGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupPathElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ManagementGroupPathElement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<ManagementGroupDetails>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<ManagementGroupChildInfo>,
}
impl ManagementGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplayProperties {
    pub fn new() -> Self {
        Self::default()
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
pub struct OperationResults {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupInfoProperties>,
}
impl OperationResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParentGroupInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ParentGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchManagementGroupRequest {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "parentGroupId", default, skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
}
impl PatchManagementGroupRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Permissions {
    #[serde(rename = "noaccess")]
    Noaccess,
    #[serde(rename = "view")]
    View,
    #[serde(rename = "edit")]
    Edit,
    #[serde(rename = "delete")]
    Delete,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionUnderManagementGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionUnderManagementGroupProperties>,
}
impl SubscriptionUnderManagementGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionUnderManagementGroupProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<DescendantParentGroupInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl SubscriptionUnderManagementGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantBackfillStatusResult {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<tenant_backfill_status_result::Status>,
}
impl TenantBackfillStatusResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod tenant_backfill_status_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        NotStarted,
        NotStartedButGroupsExist,
        Started,
        Failed,
        Cancelled,
        Completed,
    }
}
