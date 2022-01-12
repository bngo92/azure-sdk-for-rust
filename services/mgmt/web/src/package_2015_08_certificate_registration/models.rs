#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppServiceCertificate {
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
    #[serde(rename = "keyVaultSecretName", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_secret_name: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<app_service_certificate::ProvisioningState>,
}
impl AppServiceCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod app_service_certificate {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Initialized,
        WaitingOnCertificateOrder,
        Succeeded,
        CertificateOrderFailed,
        OperationNotPermittedOnKeyVault,
        AzureServiceUnauthorizedToAccessKeyVault,
        KeyVaultDoesNotExist,
        KeyVaultSecretDoesNotExist,
        UnknownError,
        ExternalPrivateKey,
        Unknown,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServiceCertificateCollection {
    pub value: Vec<AppServiceCertificateResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AppServiceCertificateCollection {
    pub fn new(value: Vec<AppServiceCertificateResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServiceCertificateOrder {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<app_service_certificate_order::Properties>,
}
impl AppServiceCertificateOrder {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
pub mod app_service_certificate_order {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub certificates: Option<serde_json::Value>,
        #[serde(rename = "distinguishedName", default, skip_serializing_if = "Option::is_none")]
        pub distinguished_name: Option<String>,
        #[serde(rename = "domainVerificationToken", default, skip_serializing_if = "Option::is_none")]
        pub domain_verification_token: Option<String>,
        #[serde(rename = "validityInYears", default, skip_serializing_if = "Option::is_none")]
        pub validity_in_years: Option<i32>,
        #[serde(rename = "keySize", default, skip_serializing_if = "Option::is_none")]
        pub key_size: Option<i32>,
        #[serde(rename = "productType")]
        pub product_type: properties::ProductType,
        #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
        pub auto_renew: Option<bool>,
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[serde(rename = "signedCertificate", default, skip_serializing_if = "Option::is_none")]
        pub signed_certificate: Option<CertificateDetails>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub csr: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub intermediate: Option<CertificateDetails>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub root: Option<CertificateDetails>,
        #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
        pub serial_number: Option<String>,
        #[serde(rename = "lastCertificateIssuanceTime", default, skip_serializing_if = "Option::is_none")]
        pub last_certificate_issuance_time: Option<String>,
        #[serde(rename = "expirationTime", default, skip_serializing_if = "Option::is_none")]
        pub expiration_time: Option<String>,
        #[serde(rename = "isPrivateKeyExternal", default, skip_serializing_if = "Option::is_none")]
        pub is_private_key_external: Option<bool>,
        #[serde(
            rename = "appServiceCertificateNotRenewableReasons",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub app_service_certificate_not_renewable_reasons: Vec<String>,
        #[serde(rename = "nextAutoRenewalTimeStamp", default, skip_serializing_if = "Option::is_none")]
        pub next_auto_renewal_time_stamp: Option<String>,
    }
    impl Properties {
        pub fn new(product_type: properties::ProductType) -> Self {
            Self {
                certificates: None,
                distinguished_name: None,
                domain_verification_token: None,
                validity_in_years: None,
                key_size: None,
                product_type,
                auto_renew: None,
                provisioning_state: None,
                status: None,
                signed_certificate: None,
                csr: None,
                intermediate: None,
                root: None,
                serial_number: None,
                last_certificate_issuance_time: None,
                expiration_time: None,
                is_private_key_external: None,
                app_service_certificate_not_renewable_reasons: Vec::new(),
                next_auto_renewal_time_stamp: None,
            }
        }
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProductType {
            StandardDomainValidatedSsl,
            StandardDomainValidatedWildCardSsl,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            InProgress,
            Deleting,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Pendingissuance,
            Issued,
            Revoked,
            Canceled,
            Denied,
            Pendingrevocation,
            PendingRekey,
            Unused,
            Expired,
            NotSubmitted,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServiceCertificateOrderCollection {
    pub value: Vec<AppServiceCertificateOrder>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AppServiceCertificateOrderCollection {
    pub fn new(value: Vec<AppServiceCertificateOrder>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppServiceCertificateOrderPatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<app_service_certificate_order_patch_resource::Properties>,
}
impl AppServiceCertificateOrderPatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod app_service_certificate_order_patch_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub certificates: Option<serde_json::Value>,
        #[serde(rename = "distinguishedName", default, skip_serializing_if = "Option::is_none")]
        pub distinguished_name: Option<String>,
        #[serde(rename = "domainVerificationToken", default, skip_serializing_if = "Option::is_none")]
        pub domain_verification_token: Option<String>,
        #[serde(rename = "validityInYears", default, skip_serializing_if = "Option::is_none")]
        pub validity_in_years: Option<i32>,
        #[serde(rename = "keySize", default, skip_serializing_if = "Option::is_none")]
        pub key_size: Option<i32>,
        #[serde(rename = "productType")]
        pub product_type: properties::ProductType,
        #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
        pub auto_renew: Option<bool>,
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[serde(rename = "signedCertificate", default, skip_serializing_if = "Option::is_none")]
        pub signed_certificate: Option<CertificateDetails>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub csr: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub intermediate: Option<CertificateDetails>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub root: Option<CertificateDetails>,
        #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
        pub serial_number: Option<String>,
        #[serde(rename = "lastCertificateIssuanceTime", default, skip_serializing_if = "Option::is_none")]
        pub last_certificate_issuance_time: Option<String>,
        #[serde(rename = "expirationTime", default, skip_serializing_if = "Option::is_none")]
        pub expiration_time: Option<String>,
        #[serde(rename = "isPrivateKeyExternal", default, skip_serializing_if = "Option::is_none")]
        pub is_private_key_external: Option<bool>,
        #[serde(
            rename = "appServiceCertificateNotRenewableReasons",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub app_service_certificate_not_renewable_reasons: Vec<String>,
        #[serde(rename = "nextAutoRenewalTimeStamp", default, skip_serializing_if = "Option::is_none")]
        pub next_auto_renewal_time_stamp: Option<String>,
    }
    impl Properties {
        pub fn new(product_type: properties::ProductType) -> Self {
            Self {
                certificates: None,
                distinguished_name: None,
                domain_verification_token: None,
                validity_in_years: None,
                key_size: None,
                product_type,
                auto_renew: None,
                provisioning_state: None,
                status: None,
                signed_certificate: None,
                csr: None,
                intermediate: None,
                root: None,
                serial_number: None,
                last_certificate_issuance_time: None,
                expiration_time: None,
                is_private_key_external: None,
                app_service_certificate_not_renewable_reasons: Vec::new(),
                next_auto_renewal_time_stamp: None,
            }
        }
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProductType {
            StandardDomainValidatedSsl,
            StandardDomainValidatedWildCardSsl,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            InProgress,
            Deleting,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Pendingissuance,
            Issued,
            Revoked,
            Canceled,
            Denied,
            Pendingrevocation,
            PendingRekey,
            Unused,
            Expired,
            NotSubmitted,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppServiceCertificatePatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppServiceCertificate>,
}
impl AppServiceCertificatePatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServiceCertificateResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppServiceCertificate>,
}
impl AppServiceCertificateResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "notBefore", default, skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(rename = "notAfter", default, skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    #[serde(rename = "signatureAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "rawData", default, skip_serializing_if = "Option::is_none")]
    pub raw_data: Option<String>,
}
impl CertificateDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateEmail {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<certificate_email::Properties>,
}
impl CertificateEmail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod certificate_email {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "emailId", default, skip_serializing_if = "Option::is_none")]
        pub email_id: Option<String>,
        #[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
        pub time_stamp: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateOrderAction {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<certificate_order_action::Properties>,
}
impl CertificateOrderAction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod certificate_order_action {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<properties::Type>,
        #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Type {
            CertificateIssued,
            CertificateOrderCanceled,
            CertificateOrderCreated,
            CertificateRevoked,
            DomainValidationComplete,
            FraudDetected,
            OrgNameChange,
            OrgValidationComplete,
            SanDrop,
            FraudCleared,
            CertificateExpired,
            CertificateExpirationWarning,
            FraudDocumentationRequired,
            Unknown,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsmOperationCollection {
    pub value: Vec<CsmOperationDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl CsmOperationCollection {
    pub fn new(value: Vec<CsmOperationDescription>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsmOperationDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<CsmOperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CsmOperationDescriptionProperties>,
}
impl CsmOperationDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsmOperationDescriptionProperties {
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl CsmOperationDescriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsmOperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CsmOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailability {
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl MetricAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "supportsInstanceLevelAggregation", default, skip_serializing_if = "Option::is_none")]
    pub supports_instance_level_aggregation: Option<bool>,
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<bool>,
    #[serde(rename = "sourceMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
    #[serde(rename = "metricFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub metric_filter_pattern: Option<String>,
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[serde(rename = "isInternal", default, skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availabilities: Vec<MetricAvailability>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameIdentifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl NameIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyOnlyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ProxyOnlyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReissueCertificateOrderRequest {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<reissue_certificate_order_request::Properties>,
}
impl ReissueCertificateOrderRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod reissue_certificate_order_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "keySize", default, skip_serializing_if = "Option::is_none")]
        pub key_size: Option<i32>,
        #[serde(rename = "delayExistingRevokeInHours", default, skip_serializing_if = "Option::is_none")]
        pub delay_existing_revoke_in_hours: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub csr: Option<String>,
        #[serde(rename = "isPrivateKeyExternal", default, skip_serializing_if = "Option::is_none")]
        pub is_private_key_external: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenewCertificateOrderRequest {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<renew_certificate_order_request::Properties>,
}
impl RenewCertificateOrderRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod renew_certificate_order_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "keySize", default, skip_serializing_if = "Option::is_none")]
        pub key_size: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub csr: Option<String>,
        #[serde(rename = "isPrivateKeyExternal", default, skip_serializing_if = "Option::is_none")]
        pub is_private_key_external: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub location: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            kind: None,
            location,
            type_: None,
            tags: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteSeal {
    pub html: String,
}
impl SiteSeal {
    pub fn new(html: String) -> Self {
        Self { html }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteSealRequest {
    #[serde(rename = "lightTheme", default, skip_serializing_if = "Option::is_none")]
    pub light_theme: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}
impl SiteSealRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
