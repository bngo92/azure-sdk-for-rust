#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "principalIdDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub principal_id_display_name: Option<String>,
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[serde(rename = "delegatedRoleDefinitionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub delegated_role_definition_ids: Vec<String>,
}
impl Authorization {
    pub fn new(principal_id: String, role_definition_id: String) -> Self {
        Self {
            principal_id,
            principal_id_display_name: None,
            role_definition_id,
            delegated_role_definition_ids: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EligibleApprover {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "principalIdDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub principal_id_display_name: Option<String>,
}
impl EligibleApprover {
    pub fn new(principal_id: String) -> Self {
        Self {
            principal_id,
            principal_id_display_name: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EligibleAuthorization {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "principalIdDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub principal_id_display_name: Option<String>,
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[serde(rename = "justInTimeAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub just_in_time_access_policy: Option<JustInTimeAccessPolicy>,
}
impl EligibleAuthorization {
    pub fn new(principal_id: String, role_definition_id: String) -> Self {
        Self {
            principal_id,
            principal_id_display_name: None,
            role_definition_id,
            just_in_time_access_policy: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JustInTimeAccessPolicy {
    #[serde(rename = "multiFactorAuthProvider")]
    pub multi_factor_auth_provider: just_in_time_access_policy::MultiFactorAuthProvider,
    #[serde(rename = "maximumActivationDuration", default, skip_serializing_if = "Option::is_none")]
    pub maximum_activation_duration: Option<String>,
    #[serde(rename = "managedByTenantApprovers", default, skip_serializing_if = "Vec::is_empty")]
    pub managed_by_tenant_approvers: Vec<EligibleApprover>,
}
impl JustInTimeAccessPolicy {
    pub fn new(multi_factor_auth_provider: just_in_time_access_policy::MultiFactorAuthProvider) -> Self {
        Self {
            multi_factor_auth_provider,
            maximum_activation_duration: None,
            managed_by_tenant_approvers: Vec::new(),
        }
    }
}
pub mod just_in_time_access_policy {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MultiFactorAuthProvider {
        Azure,
        None,
    }
    impl Default for MultiFactorAuthProvider {
        fn default() -> Self {
            Self::None
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplaceRegistrationDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MarketplaceRegistrationDefinitionProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl MarketplaceRegistrationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplaceRegistrationDefinitionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MarketplaceRegistrationDefinition>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl MarketplaceRegistrationDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceRegistrationDefinitionProperties {
    #[serde(rename = "managedByTenantId")]
    pub managed_by_tenant_id: String,
    pub authorizations: Vec<Authorization>,
    #[serde(rename = "eligibleAuthorizations", default, skip_serializing_if = "Vec::is_empty")]
    pub eligible_authorizations: Vec<EligibleAuthorization>,
    #[serde(rename = "offerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub offer_display_name: Option<String>,
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<String>,
    #[serde(rename = "planDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub plan_display_name: Option<String>,
}
impl MarketplaceRegistrationDefinitionProperties {
    pub fn new(managed_by_tenant_id: String, authorizations: Vec<Authorization>) -> Self {
        Self {
            managed_by_tenant_id,
            authorizations,
            eligible_authorizations: Vec::new(),
            offer_display_name: None,
            publisher_display_name: None,
            plan_display_name: None,
        }
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
pub struct OperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    pub publisher: String,
    pub product: String,
    pub version: String,
}
impl Plan {
    pub fn new(name: String, publisher: String, product: String, version: String) -> Self {
        Self {
            name,
            publisher,
            product,
            version,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationAssignment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistrationAssignmentProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl RegistrationAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationAssignmentList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegistrationAssignment>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RegistrationAssignmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationAssignmentProperties {
    #[serde(rename = "registrationDefinitionId")]
    pub registration_definition_id: String,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<registration_assignment_properties::ProvisioningState>,
    #[serde(rename = "registrationDefinition", default, skip_serializing_if = "Option::is_none")]
    pub registration_definition: Option<registration_assignment_properties::RegistrationDefinition>,
}
impl RegistrationAssignmentProperties {
    pub fn new(registration_definition_id: String) -> Self {
        Self {
            registration_definition_id,
            provisioning_state: None,
            registration_definition: None,
        }
    }
}
pub mod registration_assignment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Running,
        Ready,
        Creating,
        Created,
        Deleting,
        Deleted,
        Canceled,
        Failed,
        Succeeded,
        Updating,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RegistrationDefinition {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub properties: Option<registration_definition::Properties>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plan: Option<Plan>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    impl RegistrationDefinition {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod registration_definition {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Properties {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub description: Option<String>,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub authorizations: Vec<Authorization>,
            #[serde(rename = "eligibleAuthorizations", default, skip_serializing_if = "Vec::is_empty")]
            pub eligible_authorizations: Vec<EligibleAuthorization>,
            #[serde(rename = "registrationDefinitionName", default, skip_serializing_if = "Option::is_none")]
            pub registration_definition_name: Option<String>,
            #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
            pub provisioning_state: Option<properties::ProvisioningState>,
            #[serde(rename = "manageeTenantId", default, skip_serializing_if = "Option::is_none")]
            pub managee_tenant_id: Option<String>,
            #[serde(rename = "manageeTenantName", default, skip_serializing_if = "Option::is_none")]
            pub managee_tenant_name: Option<String>,
            #[serde(rename = "managedByTenantId", default, skip_serializing_if = "Option::is_none")]
            pub managed_by_tenant_id: Option<String>,
            #[serde(rename = "managedByTenantName", default, skip_serializing_if = "Option::is_none")]
            pub managed_by_tenant_name: Option<String>,
        }
        impl Properties {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod properties {
            use super::*;
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            pub enum ProvisioningState {
                NotSpecified,
                Accepted,
                Running,
                Ready,
                Creating,
                Created,
                Deleting,
                Deleted,
                Canceled,
                Failed,
                Succeeded,
                Updating,
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistrationDefinitionProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl RegistrationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationDefinitionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegistrationDefinition>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RegistrationDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationDefinitionProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub authorizations: Vec<Authorization>,
    #[serde(rename = "eligibleAuthorizations", default, skip_serializing_if = "Vec::is_empty")]
    pub eligible_authorizations: Vec<EligibleAuthorization>,
    #[serde(rename = "registrationDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub registration_definition_name: Option<String>,
    #[serde(rename = "managedByTenantId")]
    pub managed_by_tenant_id: String,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<registration_definition_properties::ProvisioningState>,
    #[serde(rename = "manageeTenantId", default, skip_serializing_if = "Option::is_none")]
    pub managee_tenant_id: Option<String>,
    #[serde(rename = "manageeTenantName", default, skip_serializing_if = "Option::is_none")]
    pub managee_tenant_name: Option<String>,
    #[serde(rename = "managedByTenantName", default, skip_serializing_if = "Option::is_none")]
    pub managed_by_tenant_name: Option<String>,
}
impl RegistrationDefinitionProperties {
    pub fn new(authorizations: Vec<Authorization>, managed_by_tenant_id: String) -> Self {
        Self {
            description: None,
            authorizations,
            eligible_authorizations: Vec::new(),
            registration_definition_name: None,
            managed_by_tenant_id,
            provisioning_state: None,
            managee_tenant_id: None,
            managee_tenant_name: None,
            managed_by_tenant_name: None,
        }
    }
}
pub mod registration_definition_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Running,
        Ready,
        Creating,
        Created,
        Deleting,
        Deleted,
        Canceled,
        Failed,
        Succeeded,
        Updating,
    }
}
