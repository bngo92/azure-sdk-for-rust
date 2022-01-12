#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureAsyncOperationResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<azure_async_operation_result::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
impl AzureAsyncOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_async_operation_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        InProgress,
        Succeeded,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureManagedOverrideRuleGroup {
    #[serde(rename = "ruleGroupOverride")]
    pub rule_group_override: azure_managed_override_rule_group::RuleGroupOverride,
    pub action: azure_managed_override_rule_group::Action,
}
impl AzureManagedOverrideRuleGroup {
    pub fn new(
        rule_group_override: azure_managed_override_rule_group::RuleGroupOverride,
        action: azure_managed_override_rule_group::Action,
    ) -> Self {
        Self {
            rule_group_override,
            action,
        }
    }
}
pub mod azure_managed_override_rule_group {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RuleGroupOverride {
        SqlInjection,
        #[serde(rename = "XSS")]
        Xss,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Allow,
        Block,
        Log,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureManagedRuleSet {
    #[serde(flatten)]
    pub managed_rule_set: ManagedRuleSet,
    #[serde(rename = "ruleGroupOverrides", default, skip_serializing_if = "Vec::is_empty")]
    pub rule_group_overrides: Vec<AzureManagedOverrideRuleGroup>,
}
impl AzureManagedRuleSet {
    pub fn new(managed_rule_set: ManagedRuleSet) -> Self {
        Self {
            managed_rule_set,
            rule_group_overrides: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Backend {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<backend::EnabledState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
    #[serde(rename = "backendHostHeader", default, skip_serializing_if = "Option::is_none")]
    pub backend_host_header: Option<String>,
}
impl Backend {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backend {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EnabledState {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendPool {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackendPoolProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl BackendPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendPoolListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BackendPool>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl BackendPoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendPoolProperties {
    #[serde(flatten)]
    pub backend_pool_update_parameters: BackendPoolUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
}
impl BackendPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendPoolUpdateParameters {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub backends: Vec<Backend>,
    #[serde(rename = "loadBalancingSettings", default, skip_serializing_if = "Option::is_none")]
    pub load_balancing_settings: Option<SubResource>,
    #[serde(rename = "healthProbeSettings", default, skip_serializing_if = "Option::is_none")]
    pub health_probe_settings: Option<SubResource>,
}
impl BackendPoolUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheConfiguration {
    #[serde(rename = "queryParameterStripDirective", default, skip_serializing_if = "Option::is_none")]
    pub query_parameter_strip_directive: Option<cache_configuration::QueryParameterStripDirective>,
    #[serde(rename = "dynamicCompression", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_compression: Option<cache_configuration::DynamicCompression>,
}
impl CacheConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cache_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueryParameterStripDirective {
        StripNone,
        StripAll,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DynamicCompression {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityInput {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: ResourceType,
}
impl CheckNameAvailabilityInput {
    pub fn new(name: String, type_: ResourceType) -> Self {
        Self { name, type_ }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityOutput {
    #[serde(rename = "nameAvailability", default, skip_serializing_if = "Option::is_none")]
    pub name_availability: Option<check_name_availability_output::NameAvailability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_output {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NameAvailability {
        Available,
        Unavailable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomHttpsConfiguration {
    #[serde(rename = "certificateSource", default, skip_serializing_if = "Option::is_none")]
    pub certificate_source: Option<custom_https_configuration::CertificateSource>,
    #[serde(rename = "protocolType", default, skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<custom_https_configuration::ProtocolType>,
    #[serde(rename = "keyVaultCertificateSourceParameters", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_certificate_source_parameters: Option<KeyVaultCertificateSourceParameters>,
    #[serde(rename = "frontDoorCertificateSourceParameters", default, skip_serializing_if = "Option::is_none")]
    pub front_door_certificate_source_parameters: Option<FrontDoorCertificateSourceParameters>,
}
impl CustomHttpsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod custom_https_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CertificateSource {
        AzureKeyVault,
        FrontDoor,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProtocolType {
        ServerNameIndication,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    pub priority: i64,
    #[serde(rename = "ruleType")]
    pub rule_type: custom_rule::RuleType,
    #[serde(rename = "rateLimitDurationInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub rate_limit_duration_in_minutes: Option<i64>,
    #[serde(rename = "rateLimitThreshold", default, skip_serializing_if = "Option::is_none")]
    pub rate_limit_threshold: Option<i64>,
    #[serde(rename = "matchConditions")]
    pub match_conditions: Vec<MatchCondition>,
    pub action: custom_rule::Action,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl CustomRule {
    pub fn new(
        priority: i64,
        rule_type: custom_rule::RuleType,
        match_conditions: Vec<MatchCondition>,
        action: custom_rule::Action,
    ) -> Self {
        Self {
            name: None,
            etag: None,
            priority,
            rule_type,
            rate_limit_duration_in_minutes: None,
            rate_limit_threshold: None,
            match_conditions,
            action,
            transforms: Vec::new(),
        }
    }
}
pub mod custom_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RuleType {
        MatchRule,
        RateLimitRule,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Allow,
        Block,
        Log,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomRules {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<CustomRule>,
}
impl CustomRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetails>,
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<String>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontDoor {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FrontDoorProperties>,
}
impl FrontDoor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontDoorCertificateSourceParameters {
    #[serde(rename = "certificateType", default, skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<front_door_certificate_source_parameters::CertificateType>,
}
impl FrontDoorCertificateSourceParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod front_door_certificate_source_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CertificateType {
        Dedicated,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontDoorListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FrontDoor>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl FrontDoorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontDoorProperties {
    #[serde(flatten)]
    pub front_door_update_parameters: FrontDoorUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
}
impl FrontDoorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontDoorUpdateParameters {
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "routingRules", default, skip_serializing_if = "Vec::is_empty")]
    pub routing_rules: Vec<RoutingRule>,
    #[serde(rename = "loadBalancingSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancing_settings: Vec<LoadBalancingSettingsModel>,
    #[serde(rename = "healthProbeSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub health_probe_settings: Vec<HealthProbeSettingsModel>,
    #[serde(rename = "backendPools", default, skip_serializing_if = "Vec::is_empty")]
    pub backend_pools: Vec<BackendPool>,
    #[serde(rename = "frontendEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub frontend_endpoints: Vec<FrontendEndpoint>,
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<front_door_update_parameters::EnabledState>,
}
impl FrontDoorUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod front_door_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EnabledState {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendEndpoint {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FrontendEndpointProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl FrontendEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendEndpointProperties {
    #[serde(flatten)]
    pub frontend_endpoint_update_parameters: FrontendEndpointUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
    #[serde(rename = "customHttpsProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_provisioning_state: Option<frontend_endpoint_properties::CustomHttpsProvisioningState>,
    #[serde(rename = "customHttpsProvisioningSubstate", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_provisioning_substate: Option<frontend_endpoint_properties::CustomHttpsProvisioningSubstate>,
    #[serde(rename = "customHttpsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_configuration: Option<CustomHttpsConfiguration>,
}
impl FrontendEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod frontend_endpoint_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomHttpsProvisioningState {
        Enabling,
        Enabled,
        Disabling,
        Disabled,
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomHttpsProvisioningSubstate {
        SubmittingDomainControlValidationRequest,
        PendingDomainControlValidationREquestApproval,
        DomainControlValidationRequestApproved,
        DomainControlValidationRequestRejected,
        DomainControlValidationRequestTimedOut,
        IssuingCertificate,
        DeployingCertificate,
        CertificateDeployed,
        DeletingCertificate,
        CertificateDeleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendEndpointUpdateParameters {
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "sessionAffinityEnabledState", default, skip_serializing_if = "Option::is_none")]
    pub session_affinity_enabled_state: Option<frontend_endpoint_update_parameters::SessionAffinityEnabledState>,
    #[serde(rename = "sessionAffinityTtlSeconds", default, skip_serializing_if = "Option::is_none")]
    pub session_affinity_ttl_seconds: Option<i64>,
    #[serde(rename = "webApplicationFirewallPolicyLink", default, skip_serializing_if = "Option::is_none")]
    pub web_application_firewall_policy_link: Option<frontend_endpoint_update_parameters::WebApplicationFirewallPolicyLink>,
}
impl FrontendEndpointUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod frontend_endpoint_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SessionAffinityEnabledState {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct WebApplicationFirewallPolicyLink {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl WebApplicationFirewallPolicyLink {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendEndpointsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FrontendEndpoint>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl FrontendEndpointsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthProbeSettingsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HealthProbeSettingsModel>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl HealthProbeSettingsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthProbeSettingsModel {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HealthProbeSettingsProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl HealthProbeSettingsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthProbeSettingsProperties {
    #[serde(flatten)]
    pub health_probe_settings_update_parameters: HealthProbeSettingsUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
}
impl HealthProbeSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthProbeSettingsUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<health_probe_settings_update_parameters::Protocol>,
    #[serde(rename = "intervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i64>,
}
impl HealthProbeSettingsUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod health_probe_settings_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        Http,
        Https,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultCertificateSourceParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault: Option<key_vault_certificate_source_parameters::Vault>,
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<String>,
}
impl KeyVaultCertificateSourceParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod key_vault_certificate_source_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Vault {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl Vault {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancingSettingsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LoadBalancingSettingsModel>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl LoadBalancingSettingsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancingSettingsModel {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LoadBalancingSettingsProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl LoadBalancingSettingsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancingSettingsProperties {
    #[serde(flatten)]
    pub load_balancing_settings_update_parameters: LoadBalancingSettingsUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
}
impl LoadBalancingSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancingSettingsUpdateParameters {
    #[serde(rename = "sampleSize", default, skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<i64>,
    #[serde(rename = "successfulSamplesRequired", default, skip_serializing_if = "Option::is_none")]
    pub successful_samples_required: Option<i64>,
    #[serde(rename = "additionalLatencyMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub additional_latency_milliseconds: Option<i64>,
}
impl LoadBalancingSettingsUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedRuleSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "ruleSetType")]
    pub rule_set_type: String,
}
impl ManagedRuleSet {
    pub fn new(rule_set_type: String) -> Self {
        Self {
            priority: None,
            version: None,
            rule_set_type,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedRuleSets {
    #[serde(rename = "ruleSets", default, skip_serializing_if = "Vec::is_empty")]
    pub rule_sets: Vec<ManagedRuleSet>,
}
impl ManagedRuleSets {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchCondition {
    #[serde(rename = "matchVariable")]
    pub match_variable: match_condition::MatchVariable,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    pub operator: match_condition::Operator,
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[serde(rename = "matchValue")]
    pub match_value: Vec<String>,
}
impl MatchCondition {
    pub fn new(match_variable: match_condition::MatchVariable, operator: match_condition::Operator, match_value: Vec<String>) -> Self {
        Self {
            match_variable,
            selector: None,
            operator,
            negate_condition: None,
            match_value,
        }
    }
}
pub mod match_condition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MatchVariable {
        RemoteAddr,
        RequestMethod,
        QueryString,
        PostArgs,
        RequestUri,
        RequestHeader,
        RequestBody,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Any,
        #[serde(rename = "IPMatch")]
        IpMatch,
        GeoMatch,
        Equal,
        Contains,
        LessThan,
        GreaterThan,
        LessThanOrEqual,
        GreaterThanOrEqual,
        BeginsWith,
        EndsWith,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurgeParameters {
    #[serde(rename = "contentPaths")]
    pub content_paths: Vec<String>,
}
impl PurgeParameters {
    pub fn new(content_paths: Vec<String>) -> Self {
        Self { content_paths }
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceState {
    Creating,
    Enabling,
    Enabled,
    Disabling,
    Disabled,
    Deleting,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    #[serde(rename = "Microsoft.Network/frontDoors")]
    MicrosoftNetworkFrontDoors,
    #[serde(rename = "Microsoft.Network/frontDoors/frontendEndpoints")]
    MicrosoftNetworkFrontDoorsFrontendEndpoints,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoutingRuleProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl RoutingRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingRuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoutingRule>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RoutingRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingRuleProperties {
    #[serde(flatten)]
    pub routing_rule_update_parameters: RoutingRuleUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
}
impl RoutingRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingRuleUpdateParameters {
    #[serde(rename = "frontendEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub frontend_endpoints: Vec<SubResource>,
    #[serde(rename = "acceptedProtocols", default, skip_serializing_if = "Vec::is_empty")]
    pub accepted_protocols: Vec<String>,
    #[serde(rename = "patternsToMatch", default, skip_serializing_if = "Vec::is_empty")]
    pub patterns_to_match: Vec<String>,
    #[serde(rename = "customForwardingPath", default, skip_serializing_if = "Option::is_none")]
    pub custom_forwarding_path: Option<String>,
    #[serde(rename = "forwardingProtocol", default, skip_serializing_if = "Option::is_none")]
    pub forwarding_protocol: Option<routing_rule_update_parameters::ForwardingProtocol>,
    #[serde(rename = "cacheConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cache_configuration: Option<CacheConfiguration>,
    #[serde(rename = "backendPool", default, skip_serializing_if = "Option::is_none")]
    pub backend_pool: Option<SubResource>,
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<routing_rule_update_parameters::EnabledState>,
}
impl RoutingRuleUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod routing_rule_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ForwardingProtocol {
        HttpOnly,
        HttpsOnly,
        MatchRequest,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EnabledState {
        Enabled,
        Disabled,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateCustomDomainInput {
    #[serde(rename = "hostName")]
    pub host_name: String,
}
impl ValidateCustomDomainInput {
    pub fn new(host_name: String) -> Self {
        Self { host_name }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateCustomDomainOutput {
    #[serde(rename = "customDomainValidated", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain_validated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ValidateCustomDomainOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationFirewallPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebApplicationFirewallPolicyPropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl WebApplicationFirewallPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationFirewallPolicyListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WebApplicationFirewallPolicy>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl WebApplicationFirewallPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationFirewallPolicyPropertiesFormat {
    #[serde(rename = "policySettings", default, skip_serializing_if = "Option::is_none")]
    pub policy_settings: Option<PolicySettings>,
    #[serde(rename = "customRules", default, skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<CustomRules>,
    #[serde(rename = "managedRules", default, skip_serializing_if = "Option::is_none")]
    pub managed_rules: Option<ManagedRuleSets>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<web_application_firewall_policy_properties_format::ResourceState>,
}
impl WebApplicationFirewallPolicyPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod web_application_firewall_policy_properties_format {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Enabling,
        Enabled,
        Disabling,
        Disabled,
        Deleting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicySettings {
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<policy_settings::EnabledState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<policy_settings::Mode>,
}
impl PolicySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EnabledState {
        Disabled,
        Enabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Prevention,
        Detection,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Transform {
    Lowercase,
    Uppercase,
    Trim,
    UrlDecode,
    UrlEncode,
    RemoveNulls,
    HtmlEntityDecode,
}
