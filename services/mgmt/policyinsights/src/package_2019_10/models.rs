#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceDetail {
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl ComplianceDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentEventDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "principalOid", default, skip_serializing_if = "Option::is_none")]
    pub principal_oid: Option<String>,
    #[serde(rename = "policyDefinitionAction", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_action: Option<String>,
}
impl ComponentEventDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentStateDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<String>,
}
impl ComponentStateDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<TypedErrorInfo>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpressionEvaluationDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "expressionKind", default, skip_serializing_if = "Option::is_none")]
    pub expression_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "expressionValue", default, skip_serializing_if = "Option::is_none")]
    pub expression_value: Option<serde_json::Value>,
    #[serde(rename = "targetValue", default, skip_serializing_if = "Option::is_none")]
    pub target_value: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
}
impl ExpressionEvaluationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IfNotExistsEvaluationDetails {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "totalResources", default, skip_serializing_if = "Option::is_none")]
    pub total_resources: Option<i64>,
}
impl IfNotExistsEvaluationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type MetadataDocument = String;
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
pub struct OperationsListResults {
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationsListResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignmentSummary {
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
    #[serde(rename = "policyDefinitions", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_definitions: Vec<PolicyDefinitionSummary>,
    #[serde(rename = "policyGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_groups: Vec<PolicyGroupSummary>,
}
impl PolicyAssignmentSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinitionSummary {
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[serde(rename = "policyDefinitionGroupNames", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_definition_group_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
}
impl PolicyDefinitionSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDetails {
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[serde(rename = "policyAssignmentDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_display_name: Option<String>,
    #[serde(rename = "policyAssignmentScope", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_scope: Option<String>,
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
}
impl PolicyDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyEvaluationDetails {
    #[serde(rename = "evaluatedExpressions", default, skip_serializing_if = "Vec::is_empty")]
    pub evaluated_expressions: Vec<ExpressionEvaluationDetails>,
    #[serde(rename = "ifNotExistsDetails", default, skip_serializing_if = "Option::is_none")]
    pub if_not_exists_details: Option<IfNotExistsEvaluationDetails>,
}
impl PolicyEvaluationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyEvent {
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[serde(rename = "effectiveParameters", default, skip_serializing_if = "Option::is_none")]
    pub effective_parameters: Option<String>,
    #[serde(rename = "isCompliant", default, skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "resourceTags", default, skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<String>,
    #[serde(rename = "policyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_name: Option<String>,
    #[serde(rename = "policyAssignmentOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_owner: Option<String>,
    #[serde(rename = "policyAssignmentParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_parameters: Option<String>,
    #[serde(rename = "policyAssignmentScope", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_scope: Option<String>,
    #[serde(rename = "policyDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_name: Option<String>,
    #[serde(rename = "policyDefinitionAction", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_action: Option<String>,
    #[serde(rename = "policyDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_category: Option<String>,
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[serde(rename = "policySetDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_name: Option<String>,
    #[serde(rename = "policySetDefinitionOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_owner: Option<String>,
    #[serde(rename = "policySetDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_category: Option<String>,
    #[serde(rename = "policySetDefinitionParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_parameters: Option<String>,
    #[serde(rename = "managementGroupIds", default, skip_serializing_if = "Option::is_none")]
    pub management_group_ids: Option<String>,
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "principalOid", default, skip_serializing_if = "Option::is_none")]
    pub principal_oid: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ComponentEventDetails>,
}
impl PolicyEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyEventsQueryResults {
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyEvent>,
}
impl PolicyEventsQueryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyGroupSummary {
    #[serde(rename = "policyGroupName", default, skip_serializing_if = "Option::is_none")]
    pub policy_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
}
impl PolicyGroupSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyMetadataProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PolicyMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyMetadataCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SlimPolicyMetadata>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PolicyMetadataCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyMetadataProperties {
    #[serde(flatten)]
    pub policy_metadata_slim_properties: PolicyMetadataSlimProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requirements: Option<String>,
}
impl PolicyMetadataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyMetadataSlimProperties {
    #[serde(rename = "metadataId", default, skip_serializing_if = "Option::is_none")]
    pub metadata_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "additionalContentUrl", default, skip_serializing_if = "Option::is_none")]
    pub additional_content_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl PolicyMetadataSlimProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyState {
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[serde(rename = "effectiveParameters", default, skip_serializing_if = "Option::is_none")]
    pub effective_parameters: Option<String>,
    #[serde(rename = "isCompliant", default, skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "resourceTags", default, skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<String>,
    #[serde(rename = "policyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_name: Option<String>,
    #[serde(rename = "policyAssignmentOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_owner: Option<String>,
    #[serde(rename = "policyAssignmentParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_parameters: Option<String>,
    #[serde(rename = "policyAssignmentScope", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_scope: Option<String>,
    #[serde(rename = "policyDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_name: Option<String>,
    #[serde(rename = "policyDefinitionAction", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_action: Option<String>,
    #[serde(rename = "policyDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_category: Option<String>,
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[serde(rename = "policySetDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_name: Option<String>,
    #[serde(rename = "policySetDefinitionOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_owner: Option<String>,
    #[serde(rename = "policySetDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_category: Option<String>,
    #[serde(rename = "policySetDefinitionParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_parameters: Option<String>,
    #[serde(rename = "managementGroupIds", default, skip_serializing_if = "Option::is_none")]
    pub management_group_ids: Option<String>,
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<String>,
    #[serde(rename = "policyEvaluationDetails", default, skip_serializing_if = "Option::is_none")]
    pub policy_evaluation_details: Option<PolicyEvaluationDetails>,
    #[serde(rename = "policyDefinitionGroupNames", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_definition_group_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ComponentStateDetails>,
    #[serde(rename = "policyDefinitionVersion", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_version: Option<String>,
    #[serde(rename = "policySetDefinitionVersion", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_version: Option<String>,
    #[serde(rename = "policyAssignmentVersion", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_version: Option<String>,
}
impl PolicyState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyStatesQueryResults {
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyState>,
}
impl PolicyStatesQueryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyTrackedResource {
    #[serde(rename = "trackedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub tracked_resource_id: Option<String>,
    #[serde(rename = "policyDetails", default, skip_serializing_if = "Option::is_none")]
    pub policy_details: Option<PolicyDetails>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<TrackedResourceModificationDetails>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<TrackedResourceModificationDetails>,
    #[serde(rename = "lastUpdateUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_update_utc: Option<String>,
}
impl PolicyTrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyTrackedResourcesQueryResults {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyTrackedResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PolicyTrackedResourcesQueryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryFailure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<query_failure::Error>,
}
impl QueryFailure {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod query_failure {
    use super::*;
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
pub struct Remediation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RemediationProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Remediation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemediationDeployment {
    #[serde(rename = "remediatedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub remediated_resource_id: Option<String>,
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
    #[serde(rename = "createdOn", default, skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "lastUpdatedOn", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_on: Option<String>,
}
impl RemediationDeployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemediationDeploymentSummary {
    #[serde(rename = "totalDeployments", default, skip_serializing_if = "Option::is_none")]
    pub total_deployments: Option<i64>,
    #[serde(rename = "successfulDeployments", default, skip_serializing_if = "Option::is_none")]
    pub successful_deployments: Option<i64>,
    #[serde(rename = "failedDeployments", default, skip_serializing_if = "Option::is_none")]
    pub failed_deployments: Option<i64>,
}
impl RemediationDeploymentSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemediationDeploymentsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RemediationDeployment>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RemediationDeploymentsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemediationFilters {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
}
impl RemediationFilters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemediationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Remediation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RemediationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemediationProperties {
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[serde(rename = "resourceDiscoveryMode", default, skip_serializing_if = "Option::is_none")]
    pub resource_discovery_mode: Option<remediation_properties::ResourceDiscoveryMode>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "createdOn", default, skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "lastUpdatedOn", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_on: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<RemediationFilters>,
    #[serde(rename = "deploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<RemediationDeploymentSummary>,
}
impl RemediationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod remediation_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceDiscoveryMode {
        ExistingNonCompliant,
        ReEvaluateCompliance,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SlimPolicyMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyMetadataSlimProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SlimPolicyMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SummarizeResults {
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Summary>,
}
impl SummarizeResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Summary {
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
    #[serde(rename = "policyAssignments", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_assignments: Vec<PolicyAssignmentSummary>,
}
impl Summary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SummaryResults {
    #[serde(rename = "queryResultsUri", default, skip_serializing_if = "Option::is_none")]
    pub query_results_uri: Option<String>,
    #[serde(rename = "nonCompliantResources", default, skip_serializing_if = "Option::is_none")]
    pub non_compliant_resources: Option<i32>,
    #[serde(rename = "nonCompliantPolicies", default, skip_serializing_if = "Option::is_none")]
    pub non_compliant_policies: Option<i32>,
    #[serde(rename = "resourceDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_details: Vec<ComplianceDetail>,
    #[serde(rename = "policyDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_details: Vec<ComplianceDetail>,
    #[serde(rename = "policyGroupDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_group_details: Vec<ComplianceDetail>,
}
impl SummaryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResourceModificationDetails {
    #[serde(rename = "policyDetails", default, skip_serializing_if = "Option::is_none")]
    pub policy_details: Option<PolicyDetails>,
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "deploymentTime", default, skip_serializing_if = "Option::is_none")]
    pub deployment_time: Option<String>,
}
impl TrackedResourceModificationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TypedErrorInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl TypedErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
