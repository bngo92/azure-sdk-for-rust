#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configuration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProperties>,
}
impl Configuration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Configuration>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProperties {
    #[serde(rename = "enforcePrivateMarkdownStorage", default, skip_serializing_if = "Option::is_none")]
    pub enforce_private_markdown_storage: Option<bool>,
}
impl ConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dashboard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DashboardProperties>,
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
impl Dashboard {
    pub fn new(location: String) -> Self {
        Self {
            properties: None,
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardLens {
    pub order: i32,
    pub parts: Vec<DashboardParts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl DashboardLens {
    pub fn new(order: i32, parts: Vec<DashboardParts>) -> Self {
        Self {
            order,
            parts,
            metadata: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DashboardListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Dashboard>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DashboardListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardPartMetadata {
    #[serde(rename = "type")]
    pub type_: String,
}
impl DashboardPartMetadata {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardParts {
    pub position: dashboard_parts::Position,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DashboardPartMetadata>,
}
impl DashboardParts {
    pub fn new(position: dashboard_parts::Position) -> Self {
        Self { position, metadata: None }
    }
}
pub mod dashboard_parts {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Position {
        pub x: i32,
        pub y: i32,
        #[serde(rename = "rowSpan")]
        pub row_span: i32,
        #[serde(rename = "colSpan")]
        pub col_span: i32,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metadata: Option<serde_json::Value>,
    }
    impl Position {
        pub fn new(x: i32, y: i32, row_span: i32, col_span: i32) -> Self {
            Self {
                x,
                y,
                row_span,
                col_span,
                metadata: None,
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DashboardProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lenses: Vec<DashboardLens>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl DashboardProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
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
pub struct MarkdownPartMetadata {
    #[serde(flatten)]
    pub dashboard_part_metadata: DashboardPartMetadata,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<markdown_part_metadata::Settings>,
}
impl MarkdownPartMetadata {
    pub fn new(dashboard_part_metadata: DashboardPartMetadata) -> Self {
        Self {
            dashboard_part_metadata,
            inputs: Vec::new(),
            settings: None,
        }
    }
}
pub mod markdown_part_metadata {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Settings {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub content: Option<settings::Content>,
    }
    impl Settings {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod settings {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Content {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub settings: Option<content::Settings>,
        }
        impl Content {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod content {
            use super::*;
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
            pub struct Settings {
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub content: Option<String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub title: Option<String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub subtitle: Option<String>,
                #[serde(rename = "markdownSource", default, skip_serializing_if = "Option::is_none")]
                pub markdown_source: Option<i32>,
                #[serde(rename = "markdownUri", default, skip_serializing_if = "Option::is_none")]
                pub markdown_uri: Option<String>,
            }
            impl Settings {
                pub fn new() -> Self {
                    Self::default()
                }
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchableDashboard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DashboardProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PatchableDashboard {
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
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<String>,
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
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ResourceProviderOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Violation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl Violation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ViolationsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Violation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ViolationsList {
    pub fn new() -> Self {
        Self::default()
    }
}
