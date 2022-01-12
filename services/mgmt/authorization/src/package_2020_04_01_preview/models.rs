#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassicAdministrator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClassicAdministratorProperties>,
}
impl ClassicAdministrator {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassicAdministratorListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClassicAdministrator>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ClassicAdministratorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassicAdministratorProperties {
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl ClassicAdministratorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DenyAssignmentProperties>,
}
impl DenyAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignmentFilter {
    #[serde(rename = "denyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub deny_assignment_name: Option<String>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "gdprExportPrincipalId", default, skip_serializing_if = "Option::is_none")]
    pub gdpr_export_principal_id: Option<String>,
}
impl DenyAssignmentFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignmentListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DenyAssignment>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DenyAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignmentPermission {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    #[serde(rename = "notActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_actions: Vec<String>,
    #[serde(rename = "dataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub data_actions: Vec<String>,
    #[serde(rename = "notDataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_data_actions: Vec<String>,
}
impl DenyAssignmentPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignmentProperties {
    #[serde(rename = "denyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub deny_assignment_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<DenyAssignmentPermission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "doNotApplyToChildScopes", default, skip_serializing_if = "Option::is_none")]
    pub do_not_apply_to_child_scopes: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub principals: Vec<Principal>,
    #[serde(rename = "excludePrincipals", default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_principals: Vec<Principal>,
    #[serde(rename = "isSystemProtected", default, skip_serializing_if = "Option::is_none")]
    pub is_system_protected: Option<bool>,
}
impl DenyAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
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
pub struct Permission {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    #[serde(rename = "notActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_actions: Vec<String>,
    #[serde(rename = "dataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub data_actions: Vec<String>,
    #[serde(rename = "notDataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_data_actions: Vec<String>,
}
impl Permission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PermissionGetResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Permission>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PermissionGetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Principal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
impl Principal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderOperation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl ProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderOperationsMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<ResourceType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<ProviderOperation>,
}
impl ProviderOperationsMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderOperationsMetadataListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProviderOperationsMetadata>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ProviderOperationsMetadataListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<ProviderOperation>,
}
impl ResourceType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentPropertiesWithScope>,
}
impl RoleAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentCreateParameters {
    pub properties: RoleAssignmentProperties,
}
impl RoleAssignmentCreateParameters {
    pub fn new(properties: RoleAssignmentProperties) -> Self {
        Self { properties }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentFilter {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "canDelegate", default, skip_serializing_if = "Option::is_none")]
    pub can_delegate: Option<bool>,
}
impl RoleAssignmentFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleAssignment>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RoleAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentMetricsResult {
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "roleAssignmentsLimit", default, skip_serializing_if = "Option::is_none")]
    pub role_assignments_limit: Option<i64>,
    #[serde(rename = "roleAssignmentsCurrentCount", default, skip_serializing_if = "Option::is_none")]
    pub role_assignments_current_count: Option<i64>,
    #[serde(rename = "roleAssignmentsRemainingCount", default, skip_serializing_if = "Option::is_none")]
    pub role_assignments_remaining_count: Option<i64>,
}
impl RoleAssignmentMetricsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentProperties {
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_assignment_properties::PrincipalType>,
    #[serde(rename = "canDelegate", default, skip_serializing_if = "Option::is_none")]
    pub can_delegate: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[serde(rename = "delegatedManagedIdentityResourceId", default, skip_serializing_if = "Option::is_none")]
    pub delegated_managed_identity_resource_id: Option<String>,
}
impl RoleAssignmentProperties {
    pub fn new(role_definition_id: String, principal_id: String) -> Self {
        Self {
            role_definition_id,
            principal_id,
            principal_type: None,
            can_delegate: None,
            description: None,
            condition: None,
            condition_version: None,
            delegated_managed_identity_resource_id: None,
        }
    }
}
pub mod role_assignment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
    }
    impl Default for PrincipalType {
        fn default() -> Self {
            Self::User
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentPropertiesWithScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_assignment_properties_with_scope::PrincipalType>,
    #[serde(rename = "canDelegate", default, skip_serializing_if = "Option::is_none")]
    pub can_delegate: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[serde(rename = "createdOn", default, skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "updatedOn", default, skip_serializing_if = "Option::is_none")]
    pub updated_on: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(rename = "delegatedManagedIdentityResourceId", default, skip_serializing_if = "Option::is_none")]
    pub delegated_managed_identity_resource_id: Option<String>,
}
impl RoleAssignmentPropertiesWithScope {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod role_assignment_properties_with_scope {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleDefinitionProperties>,
}
impl RoleDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinitionFilter {
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl RoleDefinitionFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinitionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleDefinition>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RoleDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinitionProperties {
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<Permission>,
    #[serde(rename = "assignableScopes", default, skip_serializing_if = "Vec::is_empty")]
    pub assignable_scopes: Vec<String>,
}
impl RoleDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
