#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BgpSession {
    #[serde(rename = "sessionPrefixV4", default, skip_serializing_if = "Option::is_none")]
    pub session_prefix_v4: Option<String>,
    #[serde(rename = "sessionPrefixV6", default, skip_serializing_if = "Option::is_none")]
    pub session_prefix_v6: Option<String>,
    #[serde(rename = "microsoftSessionIPv4Address", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_session_i_pv4_address: Option<String>,
    #[serde(rename = "microsoftSessionIPv6Address", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_session_i_pv6_address: Option<String>,
    #[serde(rename = "peerSessionIPv4Address", default, skip_serializing_if = "Option::is_none")]
    pub peer_session_i_pv4_address: Option<String>,
    #[serde(rename = "peerSessionIPv6Address", default, skip_serializing_if = "Option::is_none")]
    pub peer_session_i_pv6_address: Option<String>,
    #[serde(rename = "sessionStateV4", default, skip_serializing_if = "Option::is_none")]
    pub session_state_v4: Option<bgp_session::SessionStateV4>,
    #[serde(rename = "sessionStateV6", default, skip_serializing_if = "Option::is_none")]
    pub session_state_v6: Option<bgp_session::SessionStateV6>,
    #[serde(rename = "maxPrefixesAdvertisedV4", default, skip_serializing_if = "Option::is_none")]
    pub max_prefixes_advertised_v4: Option<i32>,
    #[serde(rename = "maxPrefixesAdvertisedV6", default, skip_serializing_if = "Option::is_none")]
    pub max_prefixes_advertised_v6: Option<i32>,
    #[serde(rename = "md5AuthenticationKey", default, skip_serializing_if = "Option::is_none")]
    pub md5_authentication_key: Option<String>,
}
impl BgpSession {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bgp_session {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SessionStateV4 {
        None,
        Idle,
        Connect,
        Active,
        OpenSent,
        OpenConfirm,
        OpenReceived,
        Established,
        PendingAdd,
        PendingUpdate,
        PendingRemove,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SessionStateV6 {
        None,
        Idle,
        Connect,
        Active,
        OpenSent,
        OpenConfirm,
        OpenReceived,
        Established,
        PendingAdd,
        PendingUpdate,
        PendingRemove,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CdnPeeringPrefix {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CdnPeeringPrefixProperties>,
}
impl CdnPeeringPrefix {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CdnPeeringPrefixListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CdnPeeringPrefix>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl CdnPeeringPrefixListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CdnPeeringPrefixProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "azureRegion", default, skip_serializing_if = "Option::is_none")]
    pub azure_region: Option<String>,
    #[serde(rename = "azureService", default, skip_serializing_if = "Option::is_none")]
    pub azure_service: Option<String>,
    #[serde(rename = "isPrimaryRegion", default, skip_serializing_if = "Option::is_none")]
    pub is_primary_region: Option<bool>,
    #[serde(rename = "bgpCommunity", default, skip_serializing_if = "Option::is_none")]
    pub bgp_community: Option<String>,
}
impl CdnPeeringPrefixProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckServiceProviderAvailabilityInput {
    #[serde(rename = "peeringServiceLocation", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_location: Option<String>,
    #[serde(rename = "peeringServiceProvider", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_provider: Option<String>,
}
impl CheckServiceProviderAvailabilityInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<contact_detail::Role>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl ContactDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod contact_detail {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        Noc,
        Policy,
        Technical,
        Service,
        Escalation,
        Other,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectConnection {
    #[serde(rename = "bandwidthInMbps", default, skip_serializing_if = "Option::is_none")]
    pub bandwidth_in_mbps: Option<i32>,
    #[serde(rename = "provisionedBandwidthInMbps", default, skip_serializing_if = "Option::is_none")]
    pub provisioned_bandwidth_in_mbps: Option<i32>,
    #[serde(rename = "sessionAddressProvider", default, skip_serializing_if = "Option::is_none")]
    pub session_address_provider: Option<direct_connection::SessionAddressProvider>,
    #[serde(rename = "useForPeeringService", default, skip_serializing_if = "Option::is_none")]
    pub use_for_peering_service: Option<bool>,
    #[serde(rename = "microsoftTrackingId", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_tracking_id: Option<String>,
    #[serde(rename = "peeringDBFacilityId", default, skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_id: Option<i32>,
    #[serde(rename = "connectionState", default, skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<direct_connection::ConnectionState>,
    #[serde(rename = "bgpSession", default, skip_serializing_if = "Option::is_none")]
    pub bgp_session: Option<BgpSession>,
    #[serde(rename = "connectionIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub connection_identifier: Option<String>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl DirectConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod direct_connection {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SessionAddressProvider {
        Microsoft,
        Peer,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ConnectionState {
        None,
        PendingApproval,
        Approved,
        ProvisioningStarted,
        ProvisioningFailed,
        ProvisioningCompleted,
        Validating,
        Active,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectPeeringFacility {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "directPeeringType", default, skip_serializing_if = "Option::is_none")]
    pub direct_peering_type: Option<direct_peering_facility::DirectPeeringType>,
    #[serde(rename = "peeringDBFacilityId", default, skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_id: Option<i32>,
    #[serde(rename = "peeringDBFacilityLink", default, skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_link: Option<String>,
}
impl DirectPeeringFacility {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod direct_peering_facility {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DirectPeeringType {
        Edge,
        Transit,
        Cdn,
        Internal,
        Ix,
        IxRs,
        Voice,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExchangeConnection {
    #[serde(rename = "peeringDBFacilityId", default, skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_id: Option<i32>,
    #[serde(rename = "connectionState", default, skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<exchange_connection::ConnectionState>,
    #[serde(rename = "bgpSession", default, skip_serializing_if = "Option::is_none")]
    pub bgp_session: Option<BgpSession>,
    #[serde(rename = "connectionIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub connection_identifier: Option<String>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl ExchangeConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod exchange_connection {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ConnectionState {
        None,
        PendingApproval,
        Approved,
        ProvisioningStarted,
        ProvisioningFailed,
        ProvisioningCompleted,
        Validating,
        Active,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExchangePeeringFacility {
    #[serde(rename = "exchangeName", default, skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
    #[serde(rename = "bandwidthInMbps", default, skip_serializing_if = "Option::is_none")]
    pub bandwidth_in_mbps: Option<i32>,
    #[serde(rename = "microsoftIPv4Address", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_i_pv4_address: Option<String>,
    #[serde(rename = "microsoftIPv6Address", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_i_pv6_address: Option<String>,
    #[serde(rename = "facilityIPv4Prefix", default, skip_serializing_if = "Option::is_none")]
    pub facility_i_pv4_prefix: Option<String>,
    #[serde(rename = "facilityIPv6Prefix", default, skip_serializing_if = "Option::is_none")]
    pub facility_i_pv6_prefix: Option<String>,
    #[serde(rename = "peeringDBFacilityId", default, skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_id: Option<i32>,
    #[serde(rename = "peeringDBFacilityLink", default, skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_link: Option<String>,
}
impl ExchangePeeringFacility {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeerAsn {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeerAsnProperties>,
}
impl PeerAsn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeerAsnListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeerAsn>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PeerAsnListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeerAsnProperties {
    #[serde(rename = "peerAsn", default, skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<i32>,
    #[serde(rename = "peerContactDetail", default, skip_serializing_if = "Vec::is_empty")]
    pub peer_contact_detail: Vec<ContactDetail>,
    #[serde(rename = "peerName", default, skip_serializing_if = "Option::is_none")]
    pub peer_name: Option<String>,
    #[serde(rename = "validationState", default, skip_serializing_if = "Option::is_none")]
    pub validation_state: Option<peer_asn_properties::ValidationState>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl PeerAsnProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peer_asn_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ValidationState {
        None,
        Pending,
        Approved,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Peering {
    #[serde(flatten)]
    pub resource: Resource,
    pub sku: PeeringSku,
    pub kind: peering::Kind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringProperties>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Peering {
    pub fn new(sku: PeeringSku, kind: peering::Kind, location: String) -> Self {
        Self {
            resource: Resource::default(),
            sku,
            kind,
            properties: None,
            location,
            tags: None,
        }
    }
}
pub mod peering {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Direct,
        Exchange,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringBandwidthOffer {
    #[serde(rename = "offerName", default, skip_serializing_if = "Option::is_none")]
    pub offer_name: Option<String>,
    #[serde(rename = "valueInMbps", default, skip_serializing_if = "Option::is_none")]
    pub value_in_mbps: Option<i32>,
}
impl PeeringBandwidthOffer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Peering>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PeeringListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringLocation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<peering_location::Kind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringLocationProperties>,
}
impl PeeringLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_location {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Direct,
        Exchange,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringLocationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringLocation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PeeringLocationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringLocationProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct: Option<PeeringLocationPropertiesDirect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange: Option<PeeringLocationPropertiesExchange>,
    #[serde(rename = "peeringLocation", default, skip_serializing_if = "Option::is_none")]
    pub peering_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "azureRegion", default, skip_serializing_if = "Option::is_none")]
    pub azure_region: Option<String>,
}
impl PeeringLocationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringLocationPropertiesDirect {
    #[serde(rename = "peeringFacilities", default, skip_serializing_if = "Vec::is_empty")]
    pub peering_facilities: Vec<DirectPeeringFacility>,
    #[serde(rename = "bandwidthOffers", default, skip_serializing_if = "Vec::is_empty")]
    pub bandwidth_offers: Vec<PeeringBandwidthOffer>,
}
impl PeeringLocationPropertiesDirect {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringLocationPropertiesExchange {
    #[serde(rename = "peeringFacilities", default, skip_serializing_if = "Vec::is_empty")]
    pub peering_facilities: Vec<ExchangePeeringFacility>,
}
impl PeeringLocationPropertiesExchange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct: Option<PeeringPropertiesDirect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange: Option<PeeringPropertiesExchange>,
    #[serde(rename = "peeringLocation", default, skip_serializing_if = "Option::is_none")]
    pub peering_location: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<peering_properties::ProvisioningState>,
}
impl PeeringProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringPropertiesDirect {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<DirectConnection>,
    #[serde(rename = "useForPeeringService", default, skip_serializing_if = "Option::is_none")]
    pub use_for_peering_service: Option<bool>,
    #[serde(rename = "peerAsn", default, skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<SubResource>,
    #[serde(rename = "directPeeringType", default, skip_serializing_if = "Option::is_none")]
    pub direct_peering_type: Option<peering_properties_direct::DirectPeeringType>,
}
impl PeeringPropertiesDirect {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_properties_direct {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DirectPeeringType {
        Edge,
        Transit,
        Cdn,
        Internal,
        Ix,
        IxRs,
        Voice,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringPropertiesExchange {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<ExchangeConnection>,
    #[serde(rename = "peerAsn", default, skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<SubResource>,
}
impl PeeringPropertiesExchange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringReceivedRoute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "nextHop", default, skip_serializing_if = "Option::is_none")]
    pub next_hop: Option<String>,
    #[serde(rename = "asPath", default, skip_serializing_if = "Option::is_none")]
    pub as_path: Option<String>,
    #[serde(rename = "originAsValidationState", default, skip_serializing_if = "Option::is_none")]
    pub origin_as_validation_state: Option<String>,
    #[serde(rename = "rpkiValidationState", default, skip_serializing_if = "Option::is_none")]
    pub rpki_validation_state: Option<String>,
    #[serde(rename = "trustAnchor", default, skip_serializing_if = "Option::is_none")]
    pub trust_anchor: Option<String>,
    #[serde(rename = "receivedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub received_timestamp: Option<String>,
}
impl PeeringReceivedRoute {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringReceivedRouteListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringReceivedRoute>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PeeringReceivedRouteListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringRegisteredAsn {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringRegisteredAsnProperties>,
}
impl PeeringRegisteredAsn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringRegisteredAsnListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringRegisteredAsn>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PeeringRegisteredAsnListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringRegisteredAsnProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "peeringServicePrefixKey", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_prefix_key: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<peering_registered_asn_properties::ProvisioningState>,
}
impl PeeringRegisteredAsnProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_registered_asn_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringRegisteredPrefix {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringRegisteredPrefixProperties>,
}
impl PeeringRegisteredPrefix {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringRegisteredPrefixListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringRegisteredPrefix>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PeeringRegisteredPrefixListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringRegisteredPrefixProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "prefixValidationState", default, skip_serializing_if = "Option::is_none")]
    pub prefix_validation_state: Option<peering_registered_prefix_properties::PrefixValidationState>,
    #[serde(rename = "peeringServicePrefixKey", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_prefix_key: Option<String>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<peering_registered_prefix_properties::ProvisioningState>,
}
impl PeeringRegisteredPrefixProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_registered_prefix_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrefixValidationState {
        None,
        Invalid,
        Verified,
        Failed,
        Pending,
        Warning,
        Unknown,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringService {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PeeringServiceSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringServiceProperties>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PeeringService {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            sku: None,
            properties: None,
            location,
            tags: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceCountry {
    #[serde(flatten)]
    pub resource: Resource,
}
impl PeeringServiceCountry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceCountryListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringServiceCountry>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PeeringServiceCountryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringService>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PeeringServiceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceLocation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringServiceLocationProperties>,
}
impl PeeringServiceLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceLocationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringServiceLocation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PeeringServiceLocationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceLocationProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "azureRegion", default, skip_serializing_if = "Option::is_none")]
    pub azure_region: Option<String>,
}
impl PeeringServiceLocationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServicePrefix {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringServicePrefixProperties>,
}
impl PeeringServicePrefix {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServicePrefixEvent {
    #[serde(rename = "eventTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub event_timestamp: Option<String>,
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "eventSummary", default, skip_serializing_if = "Option::is_none")]
    pub event_summary: Option<String>,
    #[serde(rename = "eventLevel", default, skip_serializing_if = "Option::is_none")]
    pub event_level: Option<String>,
    #[serde(rename = "eventDescription", default, skip_serializing_if = "Option::is_none")]
    pub event_description: Option<String>,
}
impl PeeringServicePrefixEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServicePrefixListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringServicePrefix>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PeeringServicePrefixListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServicePrefixProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "prefixValidationState", default, skip_serializing_if = "Option::is_none")]
    pub prefix_validation_state: Option<peering_service_prefix_properties::PrefixValidationState>,
    #[serde(rename = "learnedType", default, skip_serializing_if = "Option::is_none")]
    pub learned_type: Option<peering_service_prefix_properties::LearnedType>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<PeeringServicePrefixEvent>,
    #[serde(rename = "peeringServicePrefixKey", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_prefix_key: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<peering_service_prefix_properties::ProvisioningState>,
}
impl PeeringServicePrefixProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_service_prefix_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrefixValidationState {
        None,
        Invalid,
        Verified,
        Failed,
        Pending,
        Warning,
        Unknown,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LearnedType {
        None,
        ViaServiceProvider,
        ViaSession,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceProperties {
    #[serde(rename = "peeringServiceLocation", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_location: Option<String>,
    #[serde(rename = "peeringServiceProvider", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_provider: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<peering_service_properties::ProvisioningState>,
    #[serde(rename = "providerPrimaryPeeringLocation", default, skip_serializing_if = "Option::is_none")]
    pub provider_primary_peering_location: Option<String>,
    #[serde(rename = "providerBackupPeeringLocation", default, skip_serializing_if = "Option::is_none")]
    pub provider_backup_peering_location: Option<String>,
}
impl PeeringServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_service_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceProvider {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringServiceProviderProperties>,
}
impl PeeringServiceProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceProviderListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringServiceProvider>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PeeringServiceProviderListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceProviderProperties {
    #[serde(rename = "serviceProviderName", default, skip_serializing_if = "Option::is_none")]
    pub service_provider_name: Option<String>,
    #[serde(rename = "peeringLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub peering_locations: Vec<String>,
}
impl PeeringServiceProviderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PeeringServiceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<peering_sku::Tier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<peering_sku::Family>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<peering_sku::Size>,
}
impl PeeringSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        Premium,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Family {
        Direct,
        Exchange,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Size {
        Free,
        Metered,
        Unlimited,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceTags {
    pub fn new() -> Self {
        Self::default()
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
