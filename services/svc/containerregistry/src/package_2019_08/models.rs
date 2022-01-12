#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessToken {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}
impl AccessToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrErrorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
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
pub struct AcrManifests {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manifests: Vec<ManifestAttributesBase>,
}
impl AcrManifests {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Annotations {
    #[serde(rename = "org.opencontainers.image.created", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_created: Option<String>,
    #[serde(rename = "org.opencontainers.image.authors", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_authors: Option<String>,
    #[serde(rename = "org.opencontainers.image.url", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_url: Option<String>,
    #[serde(
        rename = "org.opencontainers.image.documentation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub org_opencontainers_image_documentation: Option<String>,
    #[serde(rename = "org.opencontainers.image.source", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_source: Option<String>,
    #[serde(rename = "org.opencontainers.image.version", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_version: Option<String>,
    #[serde(rename = "org.opencontainers.image.revision", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_revision: Option<String>,
    #[serde(rename = "org.opencontainers.image.vendor", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_vendor: Option<String>,
    #[serde(rename = "org.opencontainers.image.licenses", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_licenses: Option<String>,
    #[serde(rename = "org.opencontainers.image.ref.name", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_ref_name: Option<String>,
    #[serde(rename = "org.opencontainers.image.title", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_title: Option<String>,
    #[serde(rename = "org.opencontainers.image.description", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_description: Option<String>,
}
impl Annotations {
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
pub struct Descriptor {
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}
impl Descriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FsLayer {
    #[serde(rename = "blobSum", default, skip_serializing_if = "Option::is_none")]
    pub blob_sum: Option<String>,
}
impl FsLayer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct History {
    #[serde(rename = "v1Compatibility", default, skip_serializing_if = "Option::is_none")]
    pub v1_compatibility: Option<String>,
}
impl History {
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
    pub schema_version: Option<i64>,
}
impl Manifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<ManifestAttributesBase>,
}
impl ManifestAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestAttributesBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(rename = "imageSize", default, skip_serializing_if = "Option::is_none")]
    pub image_size: Option<i64>,
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
    #[serde(rename = "configMediaType", default, skip_serializing_if = "Option::is_none")]
    pub config_media_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(rename = "changeableAttributes", default, skip_serializing_if = "Option::is_none")]
    pub changeable_attributes: Option<ManifestChangeableAttributes>,
}
impl ManifestAttributesBase {
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
pub struct ManifestList {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manifests: Vec<ManifestListAttributes>,
}
impl ManifestList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestListAttributes {
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
}
impl ManifestListAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestWrapper {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manifests: Vec<ManifestListAttributes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<Descriptor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<Descriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "fsLayers", default, skip_serializing_if = "Vec::is_empty")]
    pub fs_layers: Vec<FsLayer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<History>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<ImageSignature>,
}
impl ManifestWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OciIndex {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manifests: Vec<ManifestListAttributes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}
impl OciIndex {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OciManifest {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<Descriptor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<Descriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}
impl OciManifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Platform {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "os.version", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "os.features", default, skip_serializing_if = "Vec::is_empty")]
    pub os_features: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<String>,
}
impl Platform {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefreshToken {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}
impl RefreshToken {
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
    pub manifest_count: Option<i64>,
    #[serde(rename = "tagCount", default, skip_serializing_if = "Option::is_none")]
    pub tag_count: Option<i64>,
    #[serde(rename = "changeableAttributes", default, skip_serializing_if = "Option::is_none")]
    pub changeable_attributes: Option<RepositoryChangeableAttributes>,
}
impl RepositoryAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryChangeableAttributes {
    #[serde(rename = "deleteEnabled", default, skip_serializing_if = "Option::is_none")]
    pub delete_enabled: Option<bool>,
    #[serde(rename = "writeEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_enabled: Option<bool>,
    #[serde(rename = "listEnabled", default, skip_serializing_if = "Option::is_none")]
    pub list_enabled: Option<bool>,
    #[serde(rename = "readEnabled", default, skip_serializing_if = "Option::is_none")]
    pub read_enabled: Option<bool>,
    #[serde(rename = "teleportEnabled", default, skip_serializing_if = "Option::is_none")]
    pub teleport_enabled: Option<bool>,
}
impl RepositoryChangeableAttributes {
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
    pub tag: Option<TagAttributesBase>,
}
impl TagAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagAttributesBase {
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
    pub changeable_attributes: Option<TagChangeableAttributes>,
}
impl TagAttributesBase {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagChangeableAttributes {
    #[serde(rename = "deleteEnabled", default, skip_serializing_if = "Option::is_none")]
    pub delete_enabled: Option<bool>,
    #[serde(rename = "writeEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_enabled: Option<bool>,
    #[serde(rename = "listEnabled", default, skip_serializing_if = "Option::is_none")]
    pub list_enabled: Option<bool>,
    #[serde(rename = "readEnabled", default, skip_serializing_if = "Option::is_none")]
    pub read_enabled: Option<bool>,
}
impl TagChangeableAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagAttributesBase>,
}
impl TagList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct V1Manifest {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "fsLayers", default, skip_serializing_if = "Vec::is_empty")]
    pub fs_layers: Vec<FsLayer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<History>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<ImageSignature>,
}
impl V1Manifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct V2Manifest {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<Descriptor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<Descriptor>,
}
impl V2Manifest {
    pub fn new() -> Self {
        Self::default()
    }
}
