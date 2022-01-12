#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<serde_json::Value>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MapsAccountProperties>,
}
impl MapsAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsAccountCreateParameters {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub sku: Sku,
}
impl MapsAccountCreateParameters {
    pub fn new(location: String, sku: Sku) -> Self {
        Self { location, tags: None, sku }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsAccountKeys {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl MapsAccountKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsAccountProperties {
    #[serde(rename = "x-ms-client-id", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_client_id: Option<String>,
}
impl MapsAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsAccountUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl MapsAccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsAccounts {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MapsAccount>,
}
impl MapsAccounts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsAccountsMoveRequest {
    #[serde(rename = "targetResourceGroup")]
    pub target_resource_group: String,
    #[serde(rename = "resourceIds")]
    pub resource_ids: Vec<String>,
}
impl MapsAccountsMoveRequest {
    pub fn new(target_resource_group: String, resource_ids: Vec<String>) -> Self {
        Self {
            target_resource_group,
            resource_ids,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsKeySpecification {
    #[serde(rename = "keyType")]
    pub key_type: maps_key_specification::KeyType,
}
impl MapsKeySpecification {
    pub fn new(key_type: maps_key_specification::KeyType) -> Self {
        Self { key_type }
    }
}
pub mod maps_key_specification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "secondary")]
        Secondary,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsOperations {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<serde_json::Value>,
}
impl MapsOperations {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self { name, tier: None }
    }
}
