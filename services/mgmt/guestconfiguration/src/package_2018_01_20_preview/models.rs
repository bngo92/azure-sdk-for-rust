#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ConfigurationParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationParameterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConfigurationParameter>,
}
impl ConfigurationParameterList {
    pub fn new() -> Self {
        Self::default()
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
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GuestConfigurationAssignmentProperties>,
}
impl GuestConfigurationAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationAssignmentList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GuestConfigurationAssignment>,
}
impl GuestConfigurationAssignmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationAssignmentProperties {
    #[serde(rename = "guestConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub guest_configuration: Option<GuestConfigurationNavigation>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<guest_configuration_assignment_properties::ProvisioningState>,
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<guest_configuration_assignment_properties::ComplianceStatus>,
    #[serde(rename = "complianceReason", default, skip_serializing_if = "Option::is_none")]
    pub compliance_reason: Option<String>,
    #[serde(rename = "latestReportId", default, skip_serializing_if = "Option::is_none")]
    pub latest_report_id: Option<String>,
    #[serde(rename = "assignmentHash", default, skip_serializing_if = "Option::is_none")]
    pub assignment_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl GuestConfigurationAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod guest_configuration_assignment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ComplianceStatus {
        Compliant,
        NotCompliant,
        Pending,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationAssignmentReport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "reportId", default, skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<guest_configuration_assignment_report::Type>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "lastModifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<guest_configuration_assignment_report::Status>,
    #[serde(rename = "refreshMode", default, skip_serializing_if = "Option::is_none")]
    pub refresh_mode: Option<guest_configuration_assignment_report::RefreshMode>,
    #[serde(rename = "rebootRequested", default, skip_serializing_if = "Option::is_none")]
    pub reboot_requested: Option<guest_configuration_assignment_report::RebootRequested>,
    #[serde(rename = "reportFormatVersion", default, skip_serializing_if = "Option::is_none")]
    pub report_format_version: Option<String>,
    #[serde(rename = "configurationVersion", default, skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<String>,
}
impl GuestConfigurationAssignmentReport {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod guest_configuration_assignment_report {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Consistency,
        Initial,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Compliant,
        NotCompliant,
        Pending,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RefreshMode {
        Push,
        Pull,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RebootRequested {
        True,
        False,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationAssignmentReportList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GuestConfigurationAssignmentReport>,
}
impl GuestConfigurationAssignmentReportList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationNavigation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<guest_configuration_navigation::Kind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "configurationParameter", default, skip_serializing_if = "Option::is_none")]
    pub configuration_parameter: Option<ConfigurationParameterList>,
}
impl GuestConfigurationNavigation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod guest_configuration_navigation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "DSC")]
        Dsc,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<operation::Properties>,
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
        pub status_code: Option<String>,
    }
    impl Properties {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
