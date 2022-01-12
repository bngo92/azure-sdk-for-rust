#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub details: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<availability_status::Properties>,
}
impl AvailabilityStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod availability_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "availabilityState", default, skip_serializing_if = "Option::is_none")]
        pub availability_state: Option<properties::AvailabilityState>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub summary: Option<String>,
        #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
        pub detailed_status: Option<String>,
        #[serde(rename = "reasonType", default, skip_serializing_if = "Option::is_none")]
        pub reason_type: Option<String>,
        #[serde(rename = "rootCauseAttributionTime", default, skip_serializing_if = "Option::is_none")]
        pub root_cause_attribution_time: Option<String>,
        #[serde(rename = "healthEventType", default, skip_serializing_if = "Option::is_none")]
        pub health_event_type: Option<String>,
        #[serde(rename = "healthEventCause", default, skip_serializing_if = "Option::is_none")]
        pub health_event_cause: Option<String>,
        #[serde(rename = "healthEventCategory", default, skip_serializing_if = "Option::is_none")]
        pub health_event_category: Option<String>,
        #[serde(rename = "healthEventId", default, skip_serializing_if = "Option::is_none")]
        pub health_event_id: Option<String>,
        #[serde(rename = "resolutionETA", default, skip_serializing_if = "Option::is_none")]
        pub resolution_eta: Option<String>,
        #[serde(rename = "occurredTime", default, skip_serializing_if = "Option::is_none")]
        pub occurred_time: Option<String>,
        #[serde(rename = "reasonChronicity", default, skip_serializing_if = "Option::is_none")]
        pub reason_chronicity: Option<properties::ReasonChronicity>,
        #[serde(rename = "reportedTime", default, skip_serializing_if = "Option::is_none")]
        pub reported_time: Option<String>,
        #[serde(rename = "recentlyResolved", default, skip_serializing_if = "Option::is_none")]
        pub recently_resolved: Option<properties::RecentlyResolved>,
        #[serde(rename = "recommendedActions", default, skip_serializing_if = "Vec::is_empty")]
        pub recommended_actions: Vec<RecommendedAction>,
        #[serde(rename = "serviceImpactingEvents", default, skip_serializing_if = "Vec::is_empty")]
        pub service_impacting_events: Vec<ServiceImpactingEvent>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum AvailabilityState {
            Available,
            Unavailable,
            Degraded,
            Unknown,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ReasonChronicity {
            Transient,
            Persistent,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct RecentlyResolved {
            #[serde(rename = "unavailableOccurredTime", default, skip_serializing_if = "Option::is_none")]
            pub unavailable_occurred_time: Option<String>,
            #[serde(rename = "resolvedTime", default, skip_serializing_if = "Option::is_none")]
            pub resolved_time: Option<String>,
            #[serde(rename = "unavailabilitySummary", default, skip_serializing_if = "Option::is_none")]
            pub unavailability_summary: Option<String>,
        }
        impl RecentlyResolved {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilityStatusListResult {
    pub value: Vec<AvailabilityStatus>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AvailabilityStatusListResult {
    pub fn new(value: Vec<AvailabilityStatus>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImpactedRegion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ImpactedRegion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImpactedResourceStatus {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<impacted_resource_status::Properties>,
}
impl ImpactedResourceStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod impacted_resource_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "availabilityState", default, skip_serializing_if = "Option::is_none")]
        pub availability_state: Option<properties::AvailabilityState>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub summary: Option<String>,
        #[serde(rename = "reasonType", default, skip_serializing_if = "Option::is_none")]
        pub reason_type: Option<properties::ReasonType>,
        #[serde(rename = "occurredTime", default, skip_serializing_if = "Option::is_none")]
        pub occurred_time: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum AvailabilityState {
            Available,
            Unavailable,
            Degraded,
            Unknown,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ReasonType {
            Unplanned,
            Planned,
            UserInitiated,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    pub value: Vec<Operation>,
}
impl OperationListResult {
    pub fn new(value: Vec<Operation>) -> Self {
        Self { value }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "actionUrl", default, skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    #[serde(rename = "actionUrlText", default, skip_serializing_if = "Option::is_none")]
    pub action_url_text: Option<String>,
}
impl RecommendedAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceImpactingEvent {
    #[serde(rename = "eventStartTime", default, skip_serializing_if = "Option::is_none")]
    pub event_start_time: Option<String>,
    #[serde(rename = "eventStatusLastModifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub event_status_last_modified_time: Option<String>,
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<service_impacting_event::Status>,
    #[serde(rename = "incidentProperties", default, skip_serializing_if = "Option::is_none")]
    pub incident_properties: Option<service_impacting_event::IncidentProperties>,
}
impl ServiceImpactingEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_impacting_event {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct IncidentProperties {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub service: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "incidentType", default, skip_serializing_if = "Option::is_none")]
        pub incident_type: Option<String>,
    }
    impl IncidentProperties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusBanner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<String>,
    #[serde(rename = "lastModifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
}
impl StatusBanner {
    pub fn new() -> Self {
        Self::default()
    }
}
