#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRule {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    pub condition: RuleCondition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<RuleAction>,
    #[serde(rename = "lastUpdatedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
}
impl AlertRule {
    pub fn new(name: String, is_enabled: bool, condition: RuleCondition) -> Self {
        Self {
            name,
            description: None,
            provisioning_state: None,
            is_enabled,
            condition,
            action: None,
            actions: Vec::new(),
            last_updated_time: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: AlertRule,
}
impl AlertRuleResource {
    pub fn new(resource: Resource, properties: AlertRule) -> Self {
        Self { resource, properties }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleResourceCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AlertRuleResource>,
}
impl AlertRuleResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleResourcePatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertRule>,
}
impl AlertRuleResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleNotification {
    pub operation: autoscale_notification::Operation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailNotification>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub webhooks: Vec<WebhookNotification>,
}
impl AutoscaleNotification {
    pub fn new(operation: autoscale_notification::Operation) -> Self {
        Self {
            operation,
            email: None,
            webhooks: Vec::new(),
        }
    }
}
pub mod autoscale_notification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        Scale,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleProfile {
    pub name: String,
    pub capacity: ScaleCapacity,
    pub rules: Vec<ScaleRule>,
    #[serde(rename = "fixedDate", default, skip_serializing_if = "Option::is_none")]
    pub fixed_date: Option<TimeWindow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
}
impl AutoscaleProfile {
    pub fn new(name: String, capacity: ScaleCapacity, rules: Vec<ScaleRule>) -> Self {
        Self {
            name,
            capacity,
            rules,
            fixed_date: None,
            recurrence: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSetting {
    pub profiles: Vec<AutoscaleProfile>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notifications: Vec<AutoscaleNotification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "targetResourceUri", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_uri: Option<String>,
    #[serde(rename = "targetResourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_location: Option<String>,
}
impl AutoscaleSetting {
    pub fn new(profiles: Vec<AutoscaleProfile>) -> Self {
        Self {
            profiles,
            notifications: Vec::new(),
            enabled: None,
            name: None,
            target_resource_uri: None,
            target_resource_location: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: AutoscaleSetting,
}
impl AutoscaleSettingResource {
    pub fn new(resource: Resource, properties: AutoscaleSetting) -> Self {
        Self { resource, properties }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingResourceCollection {
    pub value: Vec<AutoscaleSettingResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AutoscaleSettingResourceCollection {
    pub fn new(value: Vec<AutoscaleSettingResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoscaleSettingResourcePatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutoscaleSetting>,
}
impl AutoscaleSettingResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConditionOperator {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailNotification {
    #[serde(rename = "sendToSubscriptionAdministrator", default, skip_serializing_if = "Option::is_none")]
    pub send_to_subscription_administrator: Option<bool>,
    #[serde(rename = "sendToSubscriptionCoAdministrators", default, skip_serializing_if = "Option::is_none")]
    pub send_to_subscription_co_administrators: Option<bool>,
    #[serde(rename = "customEmails", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_emails: Vec<String>,
}
impl EmailNotification {
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
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationThresholdRuleCondition {
    #[serde(flatten)]
    pub rule_condition: RuleCondition,
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[serde(rename = "failedLocationCount")]
    pub failed_location_count: i32,
}
impl LocationThresholdRuleCondition {
    pub fn new(rule_condition: RuleCondition, failed_location_count: i32) -> Self {
        Self {
            rule_condition,
            window_size: None,
            failed_location_count,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementEventAggregationCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<ConditionOperator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
}
impl ManagementEventAggregationCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementEventRuleCondition {
    #[serde(flatten)]
    pub rule_condition: RuleCondition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<ManagementEventAggregationCondition>,
}
impl ManagementEventRuleCondition {
    pub fn new(rule_condition: RuleCondition) -> Self {
        Self {
            rule_condition,
            aggregation: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTrigger {
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[serde(rename = "metricNamespace", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[serde(rename = "metricResourceUri")]
    pub metric_resource_uri: String,
    #[serde(rename = "metricResourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub metric_resource_location: Option<String>,
    #[serde(rename = "timeGrain")]
    pub time_grain: String,
    pub statistic: metric_trigger::Statistic,
    #[serde(rename = "timeWindow")]
    pub time_window: String,
    #[serde(rename = "timeAggregation")]
    pub time_aggregation: metric_trigger::TimeAggregation,
    pub operator: metric_trigger::Operator,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<ScaleRuleMetricDimension>,
    #[serde(rename = "dividePerInstance", default, skip_serializing_if = "Option::is_none")]
    pub divide_per_instance: Option<bool>,
}
impl MetricTrigger {
    pub fn new(
        metric_name: String,
        metric_resource_uri: String,
        time_grain: String,
        statistic: metric_trigger::Statistic,
        time_window: String,
        time_aggregation: metric_trigger::TimeAggregation,
        operator: metric_trigger::Operator,
        threshold: f64,
    ) -> Self {
        Self {
            metric_name,
            metric_namespace: None,
            metric_resource_uri,
            metric_resource_location: None,
            time_grain,
            statistic,
            time_window,
            time_aggregation,
            operator,
            threshold,
            dimensions: Vec::new(),
            divide_per_instance: None,
        }
    }
}
pub mod metric_trigger {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Statistic {
        Average,
        Min,
        Max,
        Sum,
        Count,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TimeAggregation {
        Average,
        Minimum,
        Maximum,
        Total,
        Count,
        Last,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equals,
        NotEquals,
        GreaterThan,
        GreaterThanOrEqual,
        LessThan,
        LessThanOrEqual,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recurrence {
    pub frequency: recurrence::Frequency,
    pub schedule: RecurrentSchedule,
}
impl Recurrence {
    pub fn new(frequency: recurrence::Frequency, schedule: RecurrentSchedule) -> Self {
        Self { frequency, schedule }
    }
}
pub mod recurrence {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Frequency {
        None,
        Second,
        Minute,
        Hour,
        Day,
        Week,
        Month,
        Year,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurrentSchedule {
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    pub days: Vec<String>,
    pub hours: Vec<i32>,
    pub minutes: Vec<i32>,
}
impl RecurrentSchedule {
    pub fn new(time_zone: String, days: Vec<String>, hours: Vec<i32>, minutes: Vec<i32>) -> Self {
        Self {
            time_zone,
            days,
            hours,
            minutes,
        }
    }
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
pub struct RuleAction {
    #[serde(rename = "odata.type")]
    pub odata_type: String,
}
impl RuleAction {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleCondition {
    #[serde(rename = "odata.type")]
    pub odata_type: String,
    #[serde(rename = "dataSource", default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<RuleDataSource>,
}
impl RuleCondition {
    pub fn new(odata_type: String) -> Self {
        Self {
            odata_type,
            data_source: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleDataSource {
    #[serde(rename = "odata.type")]
    pub odata_type: String,
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[serde(rename = "legacyResourceId", default, skip_serializing_if = "Option::is_none")]
    pub legacy_resource_id: Option<String>,
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(rename = "metricNamespace", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
}
impl RuleDataSource {
    pub fn new(odata_type: String) -> Self {
        Self {
            odata_type,
            resource_uri: None,
            legacy_resource_id: None,
            resource_location: None,
            metric_namespace: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleEmailAction {
    #[serde(flatten)]
    pub rule_action: RuleAction,
    #[serde(rename = "sendToServiceOwners", default, skip_serializing_if = "Option::is_none")]
    pub send_to_service_owners: Option<bool>,
    #[serde(rename = "customEmails", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_emails: Vec<String>,
}
impl RuleEmailAction {
    pub fn new(rule_action: RuleAction) -> Self {
        Self {
            rule_action,
            send_to_service_owners: None,
            custom_emails: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleManagementEventClaimsDataSource {
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}
impl RuleManagementEventClaimsDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleManagementEventDataSource {
    #[serde(flatten)]
    pub rule_data_source: RuleDataSource,
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "eventSource", default, skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "resourceProviderName", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subStatus", default, skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<RuleManagementEventClaimsDataSource>,
}
impl RuleManagementEventDataSource {
    pub fn new(rule_data_source: RuleDataSource) -> Self {
        Self {
            rule_data_source,
            event_name: None,
            event_source: None,
            level: None,
            operation_name: None,
            resource_group_name: None,
            resource_provider_name: None,
            status: None,
            sub_status: None,
            claims: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleMetricDataSource {
    #[serde(flatten)]
    pub rule_data_source: RuleDataSource,
    #[serde(rename = "metricName", default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}
impl RuleMetricDataSource {
    pub fn new(rule_data_source: RuleDataSource) -> Self {
        Self {
            rule_data_source,
            metric_name: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleWebhookAction {
    #[serde(flatten)]
    pub rule_action: RuleAction,
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl RuleWebhookAction {
    pub fn new(rule_action: RuleAction) -> Self {
        Self {
            rule_action,
            service_uri: None,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleAction {
    pub direction: scale_action::Direction,
    #[serde(rename = "type")]
    pub type_: scale_action::Type,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub cooldown: String,
}
impl ScaleAction {
    pub fn new(direction: scale_action::Direction, type_: scale_action::Type, cooldown: String) -> Self {
        Self {
            direction,
            type_,
            value: None,
            cooldown,
        }
    }
}
pub mod scale_action {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Direction {
        None,
        Increase,
        Decrease,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        ChangeCount,
        PercentChangeCount,
        ExactCount,
        ServiceAllowedNextValue,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleCapacity {
    pub minimum: String,
    pub maximum: String,
    pub default: String,
}
impl ScaleCapacity {
    pub fn new(minimum: String, maximum: String, default: String) -> Self {
        Self { minimum, maximum, default }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleRule {
    #[serde(rename = "metricTrigger")]
    pub metric_trigger: MetricTrigger,
    #[serde(rename = "scaleAction")]
    pub scale_action: ScaleAction,
}
impl ScaleRule {
    pub fn new(metric_trigger: MetricTrigger, scale_action: ScaleAction) -> Self {
        Self {
            metric_trigger,
            scale_action,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleRuleMetricDimension {
    #[serde(rename = "DimensionName")]
    pub dimension_name: String,
    #[serde(rename = "Operator")]
    pub operator: scale_rule_metric_dimension::Operator,
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}
impl ScaleRuleMetricDimension {
    pub fn new(dimension_name: String, operator: scale_rule_metric_dimension::Operator, values: Vec<String>) -> Self {
        Self {
            dimension_name,
            operator,
            values,
        }
    }
}
pub mod scale_rule_metric_dimension {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equals,
        NotEquals,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThresholdRuleCondition {
    #[serde(flatten)]
    pub rule_condition: RuleCondition,
    pub operator: ConditionOperator,
    pub threshold: f64,
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[serde(rename = "timeAggregation", default, skip_serializing_if = "Option::is_none")]
    pub time_aggregation: Option<TimeAggregationOperator>,
}
impl ThresholdRuleCondition {
    pub fn new(rule_condition: RuleCondition, operator: ConditionOperator, threshold: f64) -> Self {
        Self {
            rule_condition,
            operator,
            threshold,
            window_size: None,
            time_aggregation: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TimeAggregationOperator {
    Average,
    Minimum,
    Maximum,
    Total,
    Last,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeWindow {
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    pub start: String,
    pub end: String,
}
impl TimeWindow {
    pub fn new(start: String, end: String) -> Self {
        Self {
            time_zone: None,
            start,
            end,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookNotification {
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl WebhookNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
