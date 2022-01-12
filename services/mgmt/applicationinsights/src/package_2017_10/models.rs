#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentPricingPlan {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PricingPlanProperties>,
}
impl ApplicationInsightsComponentPricingPlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EaSubscriptionMigrationDate {
    #[serde(rename = "isGrandFatherableSubscription", default, skip_serializing_if = "Option::is_none")]
    pub is_grand_fatherable_subscription: Option<bool>,
    #[serde(rename = "optedInDate", default, skip_serializing_if = "Option::is_none")]
    pub opted_in_date: Option<String>,
}
impl EaSubscriptionMigrationDate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PricingPlanProperties {
    #[serde(rename = "planType", default, skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cap: Option<f64>,
    #[serde(rename = "resetHour", default, skip_serializing_if = "Option::is_none")]
    pub reset_hour: Option<i64>,
    #[serde(rename = "warningThreshold", default, skip_serializing_if = "Option::is_none")]
    pub warning_threshold: Option<i64>,
    #[serde(rename = "stopSendNotificationWhenHitThreshold", default, skip_serializing_if = "Option::is_none")]
    pub stop_send_notification_when_hit_threshold: Option<bool>,
    #[serde(rename = "stopSendNotificationWhenHitCap", default, skip_serializing_if = "Option::is_none")]
    pub stop_send_notification_when_hit_cap: Option<bool>,
    #[serde(rename = "maxHistoryCap", default, skip_serializing_if = "Option::is_none")]
    pub max_history_cap: Option<f64>,
}
impl PricingPlanProperties {
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
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
