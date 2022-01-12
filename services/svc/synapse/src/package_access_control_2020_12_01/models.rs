#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckAccessDecision {
    #[serde(rename = "accessDecision", default, skip_serializing_if = "Option::is_none")]
    pub access_decision: Option<String>,
    #[serde(rename = "actionId", default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(rename = "roleAssignment", default, skip_serializing_if = "Option::is_none")]
    pub role_assignment: Option<RoleAssignmentDetails>,
}
impl CheckAccessDecision {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckPrincipalAccessRequest {
    pub subject: SubjectInfo,
    pub actions: Vec<RequiredAction>,
    pub scope: String,
}
impl CheckPrincipalAccessRequest {
    pub fn new(subject: SubjectInfo, actions: Vec<RequiredAction>, scope: String) -> Self {
        Self { subject, actions, scope }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckPrincipalAccessResponse {
    #[serde(rename = "AccessDecisions", default, skip_serializing_if = "Vec::is_empty")]
    pub access_decisions: Vec<CheckAccessDecision>,
}
impl CheckPrincipalAccessResponse {
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
pub struct ErrorContract {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl ErrorContract {
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
pub struct RequiredAction {
    pub id: String,
    #[serde(rename = "isDataAction")]
    pub is_data_action: bool,
}
impl RequiredAction {
    pub fn new(id: String, is_data_action: bool) -> Self {
        Self { id, is_data_action }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}
impl RoleAssignmentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentDetailsList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleAssignmentDetails>,
}
impl RoleAssignmentDetailsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentRequest {
    #[serde(rename = "roleId")]
    pub role_id: String,
    #[serde(rename = "principalId")]
    pub principal_id: String,
    pub scope: String,
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}
impl RoleAssignmentRequest {
    pub fn new(role_id: String, principal_id: String, scope: String) -> Self {
        Self {
            role_id,
            principal_id,
            scope,
            principal_type: None,
        }
    }
}
pub type RoleDefinitionsListResponse = Vec<SynapseRoleDefinition>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubjectInfo {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "groupIds", default, skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<String>,
}
impl SubjectInfo {
    pub fn new(principal_id: String) -> Self {
        Self {
            principal_id,
            group_ids: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SynapseRbacPermission {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    #[serde(rename = "notActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_actions: Vec<String>,
    #[serde(rename = "dataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub data_actions: Vec<String>,
    #[serde(rename = "notDataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_data_actions: Vec<String>,
}
impl SynapseRbacPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SynapseRoleDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isBuiltIn", default, skip_serializing_if = "Option::is_none")]
    pub is_built_in: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<SynapseRbacPermission>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(rename = "availabilityStatus", default, skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}
impl SynapseRoleDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
