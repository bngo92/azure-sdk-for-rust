#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConversionErrorCode {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "NO_ERROR")]
    NoError,
    #[serde(rename = "SERVICE_ERROR")]
    ServiceError,
    #[serde(rename = "INVALID_ASSET_URI")]
    InvalidAssetUri,
    #[serde(rename = "INVALID_JOB_ID")]
    InvalidJobId,
    #[serde(rename = "INVALID_GRAVITY")]
    InvalidGravity,
    #[serde(rename = "INVALID_SCALE")]
    InvalidScale,
    #[serde(rename = "ASSET_SIZE_TOO_LARGE")]
    AssetSizeTooLarge,
    #[serde(rename = "ASSET_DIMENSIONS_OUT_OF_BOUNDS")]
    AssetDimensionsOutOfBounds,
    #[serde(rename = "ZERO_FACES")]
    ZeroFaces,
    #[serde(rename = "INVALID_FACE_VERTICES")]
    InvalidFaceVertices,
    #[serde(rename = "ZERO_TRAJECTORIES_GENERATED")]
    ZeroTrajectoriesGenerated,
    #[serde(rename = "TOO_MANY_RIG_POSES")]
    TooManyRigPoses,
    #[serde(rename = "ASSET_CANNOT_BE_CONVERTED")]
    AssetCannotBeConverted,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}
impl ErrorResponse {
    pub fn new(error: ErrorDetail) -> Self {
        Self { error }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IngestionConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vector3>,
    #[serde(rename = "boundingBoxCenter", default, skip_serializing_if = "Option::is_none")]
    pub bounding_box_center: Option<Vector3>,
    pub gravity: Vector3,
    #[serde(rename = "keyFrameIndexes", default, skip_serializing_if = "Vec::is_empty")]
    pub key_frame_indexes: Vec<i32>,
    #[serde(rename = "gtTrajectory", default, skip_serializing_if = "Vec::is_empty")]
    pub gt_trajectory: Vec<Pose>,
    #[serde(rename = "principalAxis", default, skip_serializing_if = "Option::is_none")]
    pub principal_axis: Option<Quaternion>,
    pub scale: f32,
    #[serde(rename = "supportingPlane", default, skip_serializing_if = "Option::is_none")]
    pub supporting_plane: Option<Vector4>,
    #[serde(rename = "testTrajectory", default, skip_serializing_if = "Vec::is_empty")]
    pub test_trajectory: Vec<Pose>,
}
impl IngestionConfiguration {
    pub fn new(gravity: Vector3, scale: f32) -> Self {
        Self {
            dimensions: None,
            bounding_box_center: None,
            gravity,
            key_frame_indexes: Vec::new(),
            gt_trajectory: Vec::new(),
            principal_axis: None,
            scale,
            supporting_plane: None,
            test_trajectory: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngestionProperties {
    #[serde(rename = "clientErrorDetails", default, skip_serializing_if = "Option::is_none")]
    pub client_error_details: Option<String>,
    #[serde(rename = "serverErrorDetails", default, skip_serializing_if = "Option::is_none")]
    pub server_error_details: Option<String>,
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<ConversionErrorCode>,
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "outputModelUri", default, skip_serializing_if = "Option::is_none")]
    pub output_model_uri: Option<String>,
    #[serde(rename = "jobStatus", default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<JobStatus>,
    #[serde(rename = "assetFileType", default, skip_serializing_if = "Option::is_none")]
    pub asset_file_type: Option<String>,
    #[serde(rename = "inputAssetUri", default, skip_serializing_if = "Option::is_none")]
    pub input_asset_uri: Option<String>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ingestionConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_configuration: Option<IngestionConfiguration>,
}
impl IngestionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<InnerError>>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobStatus {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pose {
    pub rotation: Quaternion,
    pub translation: Vector3,
}
impl Pose {
    pub fn new(rotation: Quaternion, translation: Vector3) -> Self {
        Self { rotation, translation }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    #[serde(rename = "isIdentity", default, skip_serializing_if = "Option::is_none")]
    pub is_identity: Option<bool>,
}
impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x,
            y,
            z,
            w,
            is_identity: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadLocation {
    #[serde(rename = "inputAssetUri")]
    pub input_asset_uri: String,
}
impl UploadLocation {
    pub fn new(input_asset_uri: String) -> Self {
        Self { input_asset_uri }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}
