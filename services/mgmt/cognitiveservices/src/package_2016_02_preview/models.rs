#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CognitiveServicesAccountProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CognitiveServicesAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesAccountCreateParameters {
    pub sku: Sku,
    pub kind: cognitive_services_account_create_parameters::Kind,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub properties: CognitiveServicesAccountPropertiesCreateParameters,
}
impl CognitiveServicesAccountCreateParameters {
    pub fn new(
        sku: Sku,
        kind: cognitive_services_account_create_parameters::Kind,
        location: String,
        properties: CognitiveServicesAccountPropertiesCreateParameters,
    ) -> Self {
        Self {
            sku,
            kind,
            location,
            tags: None,
            properties,
        }
    }
}
pub mod cognitive_services_account_create_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Academic,
        #[serde(rename = "Bing.Autosuggest")]
        BingAutosuggest,
        #[serde(rename = "Bing.Search")]
        BingSearch,
        #[serde(rename = "Bing.Speech")]
        BingSpeech,
        #[serde(rename = "Bing.SpellCheck")]
        BingSpellCheck,
        ComputerVision,
        ContentModerator,
        Emotion,
        Face,
        #[serde(rename = "LUIS")]
        Luis,
        Recommendations,
        SpeakerRecognition,
        Speech,
        SpeechTranslation,
        TextAnalytics,
        TextTranslation,
        #[serde(rename = "WebLM")]
        WebLm,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccountEnumerateSkusResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CognitiveServicesResourceAndSku>,
}
impl CognitiveServicesAccountEnumerateSkusResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccountKeys {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}
impl CognitiveServicesAccountKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccountListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CognitiveServicesAccount>,
}
impl CognitiveServicesAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccountProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cognitive_services_account_properties::ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl CognitiveServicesAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cognitive_services_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        #[serde(rename = "ResolvingDNS")]
        ResolvingDns,
        Succeeded,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccountPropertiesCreateParameters {}
impl CognitiveServicesAccountPropertiesCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccountUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CognitiveServicesAccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesResourceAndSku {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl CognitiveServicesResourceAndSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorBody>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
}
impl ErrorBody {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegenerateKeyParameters {
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<regenerate_key_parameters::KeyName>,
}
impl RegenerateKeyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod regenerate_key_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyName {
        Key1,
        Key2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name, tier: None }
    }
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        F0,
        P0,
        P1,
        P2,
        S0,
        S1,
        S2,
        S3,
        S4,
        S5,
        S6,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Free,
        Standard,
        Premium,
    }
}
