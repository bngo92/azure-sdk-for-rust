#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrErrorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
impl AcrErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrErrors {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<AcrErrorInfo>,
}
impl AcrErrors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrManifestAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<AcrManifestAttributesBase>,
}
impl AcrManifestAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrManifestAttributesBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(rename = "changeableAttributes", default, skip_serializing_if = "Option::is_none")]
    pub changeable_attributes: Option<ChangeableAttributes>,
}
impl AcrManifestAttributesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrManifests {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manifests: Vec<AcrManifestAttributesBase>,
}
impl AcrManifests {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrRepositoryTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<AcrTagAttributesBase>,
}
impl AcrRepositoryTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrTagAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<AcrTagAttributesBase>,
}
impl AcrTagAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrTagAttributesBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed: Option<bool>,
    #[serde(rename = "changeableAttributes", default, skip_serializing_if = "Option::is_none")]
    pub changeable_attributes: Option<ChangeableAttributes>,
}
impl AcrTagAttributesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeableAttributes {
    #[serde(rename = "deleteEnabled", default, skip_serializing_if = "Option::is_none")]
    pub delete_enabled: Option<bool>,
    #[serde(rename = "writeEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_enabled: Option<bool>,
    #[serde(rename = "listEnabled", default, skip_serializing_if = "Option::is_none")]
    pub list_enabled: Option<bool>,
    #[serde(rename = "readEnabled", default, skip_serializing_if = "Option::is_none")]
    pub read_enabled: Option<bool>,
}
impl ChangeableAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedRepository {
    #[serde(rename = "manifestsDeleted", default, skip_serializing_if = "Vec::is_empty")]
    pub manifests_deleted: Vec<String>,
    #[serde(rename = "tagsDeleted", default, skip_serializing_if = "Vec::is_empty")]
    pub tags_deleted: Vec<String>,
}
impl DeletedRepository {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageHistory {
    #[serde(rename = "v1Compatibility", default, skip_serializing_if = "Option::is_none")]
    pub v1_compatibility: Option<String>,
}
impl ImageHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageLayer {
    #[serde(rename = "blobSum", default, skip_serializing_if = "Option::is_none")]
    pub blob_sum: Option<String>,
}
impl ImageLayer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageSignature {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<Jwk>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected: Option<String>,
}
impl ImageSignature {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Jwk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwk: Option<JwkHeader>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,
}
impl Jwk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JwkHeader {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}
impl JwkHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Manifest {
    #[serde(rename = "schemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "fsLayers", default, skip_serializing_if = "Vec::is_empty")]
    pub fs_layers: Vec<ImageLayer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<ImageHistory>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<ImageSignature>,
}
impl Manifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestAttributesManifest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub references: Vec<ManifestAttributesManifestReferences>,
    #[serde(rename = "quarantineTag", default, skip_serializing_if = "Option::is_none")]
    pub quarantine_tag: Option<String>,
}
impl ManifestAttributesManifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestAttributesManifestReferences {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
}
impl ManifestAttributesManifestReferences {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestChangeableAttributes {
    #[serde(rename = "deleteEnabled", default, skip_serializing_if = "Option::is_none")]
    pub delete_enabled: Option<bool>,
    #[serde(rename = "writeEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_enabled: Option<bool>,
    #[serde(rename = "listEnabled", default, skip_serializing_if = "Option::is_none")]
    pub list_enabled: Option<bool>,
    #[serde(rename = "readEnabled", default, skip_serializing_if = "Option::is_none")]
    pub read_enabled: Option<bool>,
    #[serde(rename = "quarantineState", default, skip_serializing_if = "Option::is_none")]
    pub quarantine_state: Option<String>,
    #[serde(rename = "quarantineDetails", default, skip_serializing_if = "Option::is_none")]
    pub quarantine_details: Option<String>,
}
impl ManifestChangeableAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Repositories {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
}
impl Repositories {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(rename = "manifestCount", default, skip_serializing_if = "Option::is_none")]
    pub manifest_count: Option<f64>,
    #[serde(rename = "tagCount", default, skip_serializing_if = "Option::is_none")]
    pub tag_count: Option<f64>,
    #[serde(rename = "changeableAttributes", default, skip_serializing_if = "Option::is_none")]
    pub changeable_attributes: Option<ChangeableAttributes>,
}
impl RepositoryAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
impl RepositoryTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<TagAttributesTag>,
}
impl TagAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagAttributesTag {
    #[serde(rename = "signatureRecord", default, skip_serializing_if = "Option::is_none")]
    pub signature_record: Option<String>,
}
impl TagAttributesTag {
    pub fn new() -> Self {
        Self::default()
    }
}
