#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdminKeyResult {
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl AdminKeyResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListQueryKeysResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QueryKey>,
}
impl ListQueryKeysResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryKey {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl QueryKey {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchServiceCreateOrUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SearchServiceProperties>,
}
impl SearchServiceCreateOrUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchServiceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SearchServiceResource>,
}
impl SearchServiceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchServiceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(rename = "replicaCount", default, skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    #[serde(rename = "partitionCount", default, skip_serializing_if = "Option::is_none")]
    pub partition_count: Option<i32>,
}
impl SearchServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchServiceReadableProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<search_service_readable_properties::Status>,
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<search_service_readable_properties::ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(rename = "replicaCount", default, skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    #[serde(rename = "partitionCount", default, skip_serializing_if = "Option::is_none")]
    pub partition_count: Option<i32>,
}
impl SearchServiceReadableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod search_service_readable_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "provisioning")]
        Provisioning,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "degraded")]
        Degraded,
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "error")]
        Error,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "provisioning")]
        Provisioning,
        #[serde(rename = "failed")]
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchServiceResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SearchServiceReadableProperties>,
}
impl SearchServiceResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "free")]
        Free,
        #[serde(rename = "standard")]
        Standard,
        #[serde(rename = "standard2")]
        Standard2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
