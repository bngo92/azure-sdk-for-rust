#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    pub kind: artifact::Kind,
}
impl Artifact {
    pub fn new(kind: artifact::Kind) -> Self {
        Self {
            azure_resource_base: AzureResourceBase::default(),
            kind,
        }
    }
}
pub mod artifact {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "template")]
        Template,
        #[serde(rename = "roleAssignment")]
        RoleAssignment,
        #[serde(rename = "policyAssignment")]
        PolicyAssignment,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Artifact>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ArtifactList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactPropertiesBase {
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
}
impl ArtifactPropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub identity: ManagedServiceIdentity,
    pub properties: AssignmentProperties,
}
impl Assignment {
    pub fn new(tracked_resource: TrackedResource, identity: ManagedServiceIdentity, properties: AssignmentProperties) -> Self {
        Self {
            tracked_resource,
            identity,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Assignment>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AssignmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentLockSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<assignment_lock_settings::Mode>,
}
impl AssignmentLockSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod assignment_lock_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        None,
        AllResources,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(rename = "blueprintId", default, skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
    pub parameters: ParameterValueCollection,
    #[serde(rename = "resourceGroups")]
    pub resource_groups: ResourceGroupValueCollection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AssignmentStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locks: Option<AssignmentLockSettings>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<assignment_properties::ProvisioningState>,
}
impl AssignmentProperties {
    pub fn new(parameters: ParameterValueCollection, resource_groups: ResourceGroupValueCollection) -> Self {
        Self {
            blueprint_resource_properties_base: BlueprintResourcePropertiesBase::default(),
            blueprint_id: None,
            parameters,
            resource_groups,
            status: None,
            locks: None,
            provisioning_state: None,
        }
    }
}
pub mod assignment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "validating")]
        Validating,
        #[serde(rename = "waiting")]
        Waiting,
        #[serde(rename = "deploying")]
        Deploying,
        #[serde(rename = "cancelling")]
        Cancelling,
        #[serde(rename = "locking")]
        Locking,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "deleting")]
        Deleting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentStatus {
    #[serde(flatten)]
    pub blueprint_resource_status_base: BlueprintResourceStatusBase,
}
impl AssignmentStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl AzureResourceBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Blueprint {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    pub properties: BlueprintProperties,
}
impl Blueprint {
    pub fn new(properties: BlueprintProperties) -> Self {
        Self {
            azure_resource_base: AzureResourceBase::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlueprintList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Blueprint>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl BlueprintList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlueprintProperties {
    #[serde(flatten)]
    pub shared_blueprint_properties: SharedBlueprintProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<serde_json::Value>,
}
impl BlueprintProperties {
    pub fn new() -> Self {
        Self {
            shared_blueprint_properties: SharedBlueprintProperties::default(),
            versions: None,
            layout: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlueprintResourcePropertiesBase {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl BlueprintResourcePropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlueprintResourceStatusBase {
    #[serde(rename = "timeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[serde(rename = "lastModified", default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}
impl BlueprintResourceStatusBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlueprintStatus {
    #[serde(flatten)]
    pub blueprint_resource_status_base: BlueprintResourceStatusBase,
}
impl BlueprintStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[serde(rename = "type")]
    pub type_: managed_service_identity::Type,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: managed_service_identity::Type) -> Self {
        Self {
            type_,
            principal_id: None,
            tenant_id: None,
        }
    }
}
pub mod managed_service_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinition {
    #[serde(rename = "type")]
    pub type_: parameter_definition::Type,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ParameterDefinitionMetadata>,
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_values: Vec<serde_json::Value>,
}
impl ParameterDefinition {
    pub fn new(type_: parameter_definition::Type) -> Self {
        Self {
            type_,
            metadata: None,
            default_value: None,
            allowed_values: Vec::new(),
        }
    }
}
pub mod parameter_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "array")]
        Array,
        #[serde(rename = "bool")]
        Bool,
        #[serde(rename = "int")]
        Int,
        #[serde(rename = "object")]
        Object,
        #[serde(rename = "secureObject")]
        SecureObject,
        #[serde(rename = "secureString")]
        SecureString,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterDefinitionCollection {}
impl ParameterDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterDefinitionMetadata {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "strongType", default, skip_serializing_if = "Option::is_none")]
    pub strong_type: Option<String>,
}
impl ParameterDefinitionMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterValue {
    #[serde(flatten)]
    pub parameter_value_base: ParameterValueBase,
    pub value: serde_json::Value,
}
impl ParameterValue {
    pub fn new(value: serde_json::Value) -> Self {
        Self {
            parameter_value_base: ParameterValueBase::default(),
            value,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterValueBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ParameterValueBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterValueCollection {}
impl ParameterValueCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignmentArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    pub properties: PolicyAssignmentArtifactProperties,
}
impl PolicyAssignmentArtifact {
    pub fn new(artifact: Artifact, properties: PolicyAssignmentArtifactProperties) -> Self {
        Self { artifact, properties }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignmentArtifactProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(flatten)]
    pub artifact_properties_base: ArtifactPropertiesBase,
    #[serde(rename = "policyDefinitionId")]
    pub policy_definition_id: String,
    pub parameters: ParameterValueCollection,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl PolicyAssignmentArtifactProperties {
    pub fn new(policy_definition_id: String, parameters: ParameterValueCollection) -> Self {
        Self {
            blueprint_resource_properties_base: BlueprintResourcePropertiesBase::default(),
            artifact_properties_base: ArtifactPropertiesBase::default(),
            policy_definition_id,
            parameters,
            resource_group: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishedBlueprint {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    pub properties: PublishedBlueprintProperties,
}
impl PublishedBlueprint {
    pub fn new(properties: PublishedBlueprintProperties) -> Self {
        Self {
            azure_resource_base: AzureResourceBase::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublishedBlueprintList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PublishedBlueprint>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PublishedBlueprintList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublishedBlueprintProperties {
    #[serde(flatten)]
    pub shared_blueprint_properties: SharedBlueprintProperties,
    #[serde(rename = "blueprintName", default, skip_serializing_if = "Option::is_none")]
    pub blueprint_name: Option<String>,
    #[serde(rename = "changeNotes", default, skip_serializing_if = "Option::is_none")]
    pub change_notes: Option<String>,
}
impl PublishedBlueprintProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ParameterDefinitionMetadata>,
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
}
impl ResourceGroupDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupDefinitionCollection {}
impl ResourceGroupDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ResourceGroupValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupValueCollection {}
impl ResourceGroupValueCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_provider_operation::Display>,
}
impl ResourceProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider_operation {
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
pub struct ResourceProviderOperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperation>,
}
impl ResourceProviderOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    pub properties: RoleAssignmentArtifactProperties,
}
impl RoleAssignmentArtifact {
    pub fn new(artifact: Artifact, properties: RoleAssignmentArtifactProperties) -> Self {
        Self { artifact, properties }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentArtifactProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(flatten)]
    pub artifact_properties_base: ArtifactPropertiesBase,
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[serde(rename = "principalIds")]
    pub principal_ids: serde_json::Value,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl RoleAssignmentArtifactProperties {
    pub fn new(role_definition_id: String, principal_ids: serde_json::Value) -> Self {
        Self {
            blueprint_resource_properties_base: BlueprintResourcePropertiesBase::default(),
            artifact_properties_base: ArtifactPropertiesBase::default(),
            role_definition_id,
            principal_ids,
            resource_group: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretReferenceParameterValue {
    #[serde(flatten)]
    pub parameter_value_base: ParameterValueBase,
    pub reference: SecretValueReference,
}
impl SecretReferenceParameterValue {
    pub fn new(reference: SecretValueReference) -> Self {
        Self {
            parameter_value_base: ParameterValueBase::default(),
            reference,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretValueReference {
    #[serde(rename = "keyVault")]
    pub key_vault: KeyVaultReference,
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<String>,
}
impl SecretValueReference {
    pub fn new(key_vault: KeyVaultReference, secret_name: String) -> Self {
        Self {
            key_vault,
            secret_name,
            secret_version: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedBlueprintProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<BlueprintStatus>,
    #[serde(rename = "targetScope", default, skip_serializing_if = "Option::is_none")]
    pub target_scope: Option<shared_blueprint_properties::TargetScope>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefinitionCollection>,
    #[serde(rename = "resourceGroups", default, skip_serializing_if = "Option::is_none")]
    pub resource_groups: Option<ResourceGroupDefinitionCollection>,
}
impl SharedBlueprintProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod shared_blueprint_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TargetScope {
        #[serde(rename = "subscription")]
        Subscription,
        #[serde(rename = "managementGroup")]
        ManagementGroup,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    pub properties: TemplateArtifactProperties,
}
impl TemplateArtifact {
    pub fn new(artifact: Artifact, properties: TemplateArtifactProperties) -> Self {
        Self { artifact, properties }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateArtifactProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(flatten)]
    pub artifact_properties_base: ArtifactPropertiesBase,
    pub template: serde_json::Value,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    pub parameters: ParameterValueCollection,
}
impl TemplateArtifactProperties {
    pub fn new(template: serde_json::Value, parameters: ParameterValueCollection) -> Self {
        Self {
            blueprint_resource_properties_base: BlueprintResourcePropertiesBase::default(),
            artifact_properties_base: ArtifactPropertiesBase::default(),
            template,
            resource_group: None,
            parameters,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            azure_resource_base: AzureResourceBase::default(),
            location,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultReference {
    pub id: String,
}
impl KeyVaultReference {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
