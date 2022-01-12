#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedReservationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AppliedReservationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedReservations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppliedReservationsProperties>,
}
impl AppliedReservations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedReservationsProperties {
    #[serde(rename = "reservationOrderIds", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_ids: Option<AppliedReservationList>,
}
impl AppliedReservationsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedScopeProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl AppliedScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppliedScopeType {
    Single,
    Shared,
}
pub type AppliedScopes = Vec<String>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableScopeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableScopeRequestProperties>,
}
impl AvailableScopeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableScopeRequestProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}
impl AvailableScopeRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type BillingScopeId = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculatePriceResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CalculatePriceResponseProperties>,
}
impl CalculatePriceResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculatePriceResponseProperties {
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<calculate_price_response_properties::BillingCurrencyTotal>,
    #[serde(rename = "netTotal", default, skip_serializing_if = "Option::is_none")]
    pub net_total: Option<f64>,
    #[serde(rename = "taxTotal", default, skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<f64>,
    #[serde(rename = "grandTotal", default, skip_serializing_if = "Option::is_none")]
    pub grand_total: Option<f64>,
    #[serde(rename = "isBillingPartnerManaged", default, skip_serializing_if = "Option::is_none")]
    pub is_billing_partner_managed: Option<bool>,
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[serde(rename = "skuTitle", default, skip_serializing_if = "Option::is_none")]
    pub sku_title: Option<String>,
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<calculate_price_response_properties::PricingCurrencyTotal>,
    #[serde(rename = "paymentSchedule", default, skip_serializing_if = "Vec::is_empty")]
    pub payment_schedule: Vec<PaymentDetail>,
}
impl CalculatePriceResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod calculate_price_response_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct BillingCurrencyTotal {
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    impl BillingCurrencyTotal {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct PricingCurrencyTotal {
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    impl PricingCurrencyTotal {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Catalog {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "billingPlans", default, skip_serializing_if = "Option::is_none")]
    pub billing_plans: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msrp: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub terms: Vec<ReservationTerm>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(rename = "skuProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub sku_properties: Vec<SkuProperty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<SkuRestriction>,
}
impl Catalog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ExtendedErrorInfo>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ErrorResponseCode {
    NotSpecified,
    InternalServerError,
    ServerTimeout,
    AuthorizationFailed,
    BadRequest,
    ClientCertificateThumbprintNotSet,
    InvalidRequestContent,
    OperationFailed,
    HttpMethodNotSupported,
    InvalidRequestUri,
    MissingTenantId,
    InvalidTenantId,
    InvalidReservationOrderId,
    InvalidReservationId,
    ReservationIdNotInReservationOrder,
    ReservationOrderNotFound,
    InvalidSubscriptionId,
    InvalidAccessToken,
    InvalidLocationId,
    UnauthenticatedRequestsThrottled,
    InvalidHealthCheckType,
    Forbidden,
    BillingScopeIdCannotBeChanged,
    AppliedScopesNotAssociatedWithCommerceAccount,
    PatchValuesSameAsExisting,
    RoleAssignmentCreationFailed,
    ReservationOrderCreationFailed,
    ReservationOrderNotEnabled,
    CapacityUpdateScopesFailed,
    UnsupportedReservationTerm,
    ReservationOrderIdAlreadyExists,
    RiskCheckFailed,
    CreateQuoteFailed,
    ActivateQuoteFailed,
    NonsupportedAccountId,
    PaymentInstrumentNotFound,
    MissingAppliedScopesForSingle,
    NoValidReservationsToReRate,
    #[serde(rename = "ReRateOnlyAllowedForEA")]
    ReRateOnlyAllowedForEa,
    OperationCannotBePerformedInCurrentState,
    InvalidSingleAppliedScopesCount,
    InvalidFulfillmentRequestParameters,
    NotSupportedCountry,
    InvalidRefundQuantity,
    PurchaseError,
    BillingCustomerInputError,
    BillingPaymentInstrumentSoftError,
    BillingPaymentInstrumentHardError,
    BillingTransientError,
    BillingError,
    FulfillmentConfigurationError,
    FulfillmentOutOfStockError,
    FulfillmentTransientError,
    FulfillmentError,
    CalculatePriceFailed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedErrorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorResponseCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExtendedErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedStatusInfo {
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<ReservationStatusCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExtendedStatusInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InstanceFlexibility {
    On,
    Off,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MergeProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<String>,
}
impl MergeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MergeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MergeProperties>,
}
impl MergeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsrpProperty {
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}
impl MsrpProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResponse>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Patch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PatchProperties>,
}
impl Patch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchProperties {
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<patch_properties::RenewProperties>,
}
impl PatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RenewProperties {
        #[serde(rename = "purchaseProperties", default, skip_serializing_if = "Option::is_none")]
        pub purchase_properties: Option<PurchaseRequest>,
    }
    impl RenewProperties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentDetail {
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "paymentDate", default, skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
    #[serde(rename = "billingAccount", default, skip_serializing_if = "Option::is_none")]
    pub billing_account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
}
impl PaymentDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PaymentStatus {
    Succeeded,
    Failed,
    Scheduled,
    Cancelled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Price {
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}
impl Price {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Properties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionScopeProperties>,
}
impl Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurchaseRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PurchaseRequestProperties>,
}
impl PurchaseRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurchaseRequestProperties {
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<ReservationQuantity>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "reservedResourceProperties", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_properties: Option<purchase_request_properties::ReservedResourceProperties>,
}
impl PurchaseRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod purchase_request_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ReservedResourceProperties {
        #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
        pub instance_flexibility: Option<InstanceFlexibility>,
    }
    impl ReservedResourceProperties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
pub type Renew = bool;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenewPropertiesResponse {
    #[serde(rename = "purchaseProperties", default, skip_serializing_if = "Option::is_none")]
    pub purchase_properties: Option<PurchaseRequest>,
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<renew_properties_response::PricingCurrencyTotal>,
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<renew_properties_response::BillingCurrencyTotal>,
}
impl RenewPropertiesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod renew_properties_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct PricingCurrencyTotal {
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    impl PricingCurrencyTotal {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct BillingCurrencyTotal {
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    impl BillingCurrencyTotal {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservationBillingPlan {
    Upfront,
    Monthly,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationResponse>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ReservationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationMergeProperties {
    #[serde(rename = "mergeDestination", default, skip_serializing_if = "Option::is_none")]
    pub merge_destination: Option<String>,
    #[serde(rename = "mergeSources", default, skip_serializing_if = "Vec::is_empty")]
    pub merge_sources: Vec<String>,
}
impl ReservationMergeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderBillingPlanInformation {
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "nextPaymentDueDate", default, skip_serializing_if = "Option::is_none")]
    pub next_payment_due_date: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<PaymentDetail>,
}
impl ReservationOrderBillingPlanInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationOrderResponse>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ReservationOrderList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderProperties {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "requestDateTime", default, skip_serializing_if = "Option::is_none")]
    pub request_date_time: Option<String>,
    #[serde(rename = "createdDateTime", default, skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(rename = "originalQuantity", default, skip_serializing_if = "Option::is_none")]
    pub original_quantity: Option<ReservationQuantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[serde(rename = "planInformation", default, skip_serializing_if = "Option::is_none")]
    pub plan_information: Option<ReservationOrderBillingPlanInformation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reservations: Vec<ReservationResponse>,
}
impl ReservationOrderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationOrderProperties>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ReservationOrderResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationProperties {
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<ReservationQuantity>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "effectiveDateTime", default, skip_serializing_if = "Option::is_none")]
    pub effective_date_time: Option<String>,
    #[serde(rename = "lastUpdatedDateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<String>,
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[serde(rename = "splitProperties", default, skip_serializing_if = "Option::is_none")]
    pub split_properties: Option<ReservationSplitProperties>,
    #[serde(rename = "mergeProperties", default, skip_serializing_if = "Option::is_none")]
    pub merge_properties: Option<ReservationMergeProperties>,
    #[serde(rename = "swapProperties", default, skip_serializing_if = "Option::is_none")]
    pub swap_properties: Option<ReservationSwapProperties>,
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "renewSource", default, skip_serializing_if = "Option::is_none")]
    pub renew_source: Option<String>,
    #[serde(rename = "renewDestination", default, skip_serializing_if = "Option::is_none")]
    pub renew_destination: Option<String>,
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<RenewPropertiesResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<String>,
}
impl ReservationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ReservationQuantity = i32;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationProperties>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ReservationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSplitProperties {
    #[serde(rename = "splitDestinations", default, skip_serializing_if = "Vec::is_empty")]
    pub split_destinations: Vec<String>,
    #[serde(rename = "splitSource", default, skip_serializing_if = "Option::is_none")]
    pub split_source: Option<String>,
}
impl ReservationSplitProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservationStatusCode {
    None,
    Pending,
    Active,
    PurchaseError,
    PaymentInstrumentError,
    Split,
    Merged,
    Expired,
    Succeeded,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSwapProperties {
    #[serde(rename = "swapSource", default, skip_serializing_if = "Option::is_none")]
    pub swap_source: Option<String>,
    #[serde(rename = "swapDestination", default, skip_serializing_if = "Option::is_none")]
    pub swap_destination: Option<String>,
}
impl ReservationSwapProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservationTerm {
    #[serde(rename = "P1Y")]
    P1y,
    #[serde(rename = "P3Y")]
    P3y,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservedResourceType {
    VirtualMachines,
    SqlDatabases,
    SuseLinux,
    CosmosDb,
    RedHat,
    SqlDataWarehouse,
    VMwareCloudSimple,
    RedHatOsa,
    Databricks,
    AppService,
    ManagedDisk,
    BlockBlob,
    RedisCache,
    AzureDataExplorer,
    MySql,
    MariaDb,
    PostgreSql,
    DedicatedHost,
    SapHana,
    SqlAzureHybridBenefit,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SkuName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuRestriction {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}
impl SkuRestriction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SplitProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub quantities: Vec<i64>,
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
}
impl SplitProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SplitRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SplitProperties>,
}
impl SplitRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionScopeProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<ScopeProperties>,
}
impl SubscriptionScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
