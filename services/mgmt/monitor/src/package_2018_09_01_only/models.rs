#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    #[serde(rename = "groupShortName")]
    pub group_short_name: String,
    pub enabled: bool,
    #[serde(rename = "emailReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub email_receivers: Vec<EmailReceiver>,
    #[serde(rename = "smsReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub sms_receivers: Vec<SmsReceiver>,
    #[serde(rename = "webhookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub webhook_receivers: Vec<WebhookReceiver>,
    #[serde(rename = "itsmReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub itsm_receivers: Vec<ItsmReceiver>,
    #[serde(rename = "azureAppPushReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_app_push_receivers: Vec<AzureAppPushReceiver>,
    #[serde(rename = "automationRunbookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub automation_runbook_receivers: Vec<AutomationRunbookReceiver>,
    #[serde(rename = "voiceReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub voice_receivers: Vec<VoiceReceiver>,
    #[serde(rename = "logicAppReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub logic_app_receivers: Vec<LogicAppReceiver>,
    #[serde(rename = "azureFunctionReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_function_receivers: Vec<AzureFunctionReceiver>,
    #[serde(rename = "armRoleReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub arm_role_receivers: Vec<ArmRoleReceiver>,
}
impl ActionGroup {
    pub fn new(group_short_name: String, enabled: bool) -> Self {
        Self {
            group_short_name,
            enabled,
            email_receivers: Vec::new(),
            sms_receivers: Vec::new(),
            webhook_receivers: Vec::new(),
            itsm_receivers: Vec::new(),
            azure_app_push_receivers: Vec::new(),
            automation_runbook_receivers: Vec::new(),
            voice_receivers: Vec::new(),
            logic_app_receivers: Vec::new(),
            azure_function_receivers: Vec::new(),
            arm_role_receivers: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ActionGroupResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ActionGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ActionGroupPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupPatchBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroupPatch>,
}
impl ActionGroupPatchBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroup>,
}
impl ActionGroupResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmRoleReceiver {
    pub name: String,
    #[serde(rename = "roleId")]
    pub role_id: String,
}
impl ArmRoleReceiver {
    pub fn new(name: String, role_id: String) -> Self {
        Self { name, role_id }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRunbookReceiver {
    #[serde(rename = "automationAccountId")]
    pub automation_account_id: String,
    #[serde(rename = "runbookName")]
    pub runbook_name: String,
    #[serde(rename = "webhookResourceId")]
    pub webhook_resource_id: String,
    #[serde(rename = "isGlobalRunbook")]
    pub is_global_runbook: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
}
impl AutomationRunbookReceiver {
    pub fn new(automation_account_id: String, runbook_name: String, webhook_resource_id: String, is_global_runbook: bool) -> Self {
        Self {
            automation_account_id,
            runbook_name,
            webhook_resource_id,
            is_global_runbook,
            name: None,
            service_uri: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAppPushReceiver {
    pub name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}
impl AzureAppPushReceiver {
    pub fn new(name: String, email_address: String) -> Self {
        Self { name, email_address }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFunctionReceiver {
    pub name: String,
    #[serde(rename = "functionAppResourceId")]
    pub function_app_resource_id: String,
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "httpTriggerUrl")]
    pub http_trigger_url: String,
}
impl AzureFunctionReceiver {
    pub fn new(name: String, function_app_resource_id: String, function_name: String, http_trigger_url: String) -> Self {
        Self {
            name,
            function_app_resource_id,
            function_name,
            http_trigger_url,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Baseline {
    pub sensitivity: baseline::Sensitivity,
    #[serde(rename = "lowThresholds")]
    pub low_thresholds: Vec<f64>,
    #[serde(rename = "highThresholds")]
    pub high_thresholds: Vec<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
    #[serde(rename = "PredictionResultType", default, skip_serializing_if = "Option::is_none")]
    pub prediction_result_type: Option<baseline::PredictionResultType>,
    #[serde(rename = "ErrorType", default, skip_serializing_if = "Option::is_none")]
    pub error_type: Option<baseline::ErrorType>,
}
impl Baseline {
    pub fn new(sensitivity: baseline::Sensitivity, low_thresholds: Vec<f64>, high_thresholds: Vec<f64>) -> Self {
        Self {
            sensitivity,
            low_thresholds,
            high_thresholds,
            timestamps: Vec::new(),
            prediction_result_type: None,
            error_type: None,
        }
    }
}
pub mod baseline {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Sensitivity {
        Low,
        Medium,
        High,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PredictionResultType {}
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ErrorType {}
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaselineMetadataValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl BaselineMetadataValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaselineProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timespan: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(rename = "internalOperationId", default, skip_serializing_if = "Option::is_none")]
    pub internal_operation_id: Option<String>,
}
impl BaselineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaselineResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub baseline: Vec<Baseline>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metdata: Vec<BaselineMetadataValue>,
    #[serde(rename = "predictionResultType", default, skip_serializing_if = "Option::is_none")]
    pub prediction_result_type: Option<baseline_response::PredictionResultType>,
    #[serde(rename = "errorType", default, skip_serializing_if = "Option::is_none")]
    pub error_type: Option<baseline_response::ErrorType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BaselineProperties>,
}
impl BaselineResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod baseline_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PredictionResultType {}
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ErrorType {}
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalculateBaselineResponse {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
    pub baseline: Vec<Baseline>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<calculate_baseline_response::Statistics>,
    #[serde(rename = "internalOperationId", default, skip_serializing_if = "Option::is_none")]
    pub internal_operation_id: Option<String>,
    #[serde(rename = "errorType", default, skip_serializing_if = "Option::is_none")]
    pub error_type: Option<calculate_baseline_response::ErrorType>,
}
impl CalculateBaselineResponse {
    pub fn new(type_: String, baseline: Vec<Baseline>) -> Self {
        Self {
            type_,
            timestamps: Vec::new(),
            baseline,
            statistics: None,
            internal_operation_id: None,
            error_type: None,
        }
    }
}
pub mod calculate_baseline_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Statistics {
        #[serde(rename = "isEligible", default, skip_serializing_if = "Option::is_none")]
        pub is_eligible: Option<bool>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub status: Vec<String>,
        #[serde(rename = "seasonalityPeriod", default, skip_serializing_if = "Option::is_none")]
        pub seasonality_period: Option<i32>,
    }
    impl Statistics {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ErrorType {}
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailReceiver {
    pub name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
impl EmailReceiver {
    pub fn new(name: String, email_address: String) -> Self {
        Self {
            name,
            email_address,
            status: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableRequest {
    #[serde(rename = "receiverName")]
    pub receiver_name: String,
}
impl EnableRequest {
    pub fn new(receiver_name: String) -> Self {
        Self { receiver_name }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItsmReceiver {
    pub name: String,
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[serde(rename = "ticketConfiguration")]
    pub ticket_configuration: String,
    pub region: String,
}
impl ItsmReceiver {
    pub fn new(name: String, workspace_id: String, connection_id: String, ticket_configuration: String, region: String) -> Self {
        Self {
            name,
            workspace_id,
            connection_id,
            ticket_configuration,
            region,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizableString {
    pub value: String,
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl LocalizableString {
    pub fn new(value: String) -> Self {
        Self {
            value,
            localized_value: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogicAppReceiver {
    pub name: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
}
impl LogicAppReceiver {
    pub fn new(name: String, resource_id: String, callback_url: String) -> Self {
        Self {
            name,
            resource_id,
            callback_url,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReceiverStatus {
    NotSpecified,
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsReceiver {
    pub name: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
impl SmsReceiver {
    pub fn new(name: String, country_code: String, phone_number: String) -> Self {
        Self {
            name,
            country_code,
            phone_number,
            status: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesInformation {
    pub sensitivities: Vec<String>,
    pub values: Vec<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
}
impl TimeSeriesInformation {
    pub fn new(sensitivities: Vec<String>, values: Vec<f64>) -> Self {
        Self {
            sensitivities,
            values,
            timestamps: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoiceReceiver {
    pub name: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
}
impl VoiceReceiver {
    pub fn new(name: String, country_code: String, phone_number: String) -> Self {
        Self {
            name,
            country_code,
            phone_number,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookReceiver {
    pub name: String,
    #[serde(rename = "serviceUri")]
    pub service_uri: String,
}
impl WebhookReceiver {
    pub fn new(name: String, service_uri: String) -> Self {
        Self { name, service_uri }
    }
}
