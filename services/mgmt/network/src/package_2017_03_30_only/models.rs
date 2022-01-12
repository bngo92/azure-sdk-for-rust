#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGatewayBackendAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl ApplicationGatewayBackendAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGatewayBackendAddressPool {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationGatewayBackendAddressPoolPropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ApplicationGatewayBackendAddressPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGatewayBackendAddressPoolPropertiesFormat {
    #[serde(rename = "backendIPConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub backend_ip_configurations: Vec<NetworkInterfaceIpConfiguration>,
    #[serde(rename = "backendAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub backend_addresses: Vec<ApplicationGatewayBackendAddress>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ApplicationGatewayBackendAddressPoolPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationSecurityGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationSecurityGroupPropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ApplicationSecurityGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationSecurityGroupPropertiesFormat {
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ApplicationSecurityGroupPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendAddressPool {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackendAddressPoolPropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl BackendAddressPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendAddressPoolPropertiesFormat {
    #[serde(rename = "backendIPConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub backend_ip_configurations: Vec<NetworkInterfaceIpConfiguration>,
    #[serde(rename = "loadBalancingRules", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancing_rules: Vec<SubResource>,
    #[serde(rename = "outboundNatRule", default, skip_serializing_if = "Option::is_none")]
    pub outbound_nat_rule: Option<SubResource>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl BackendAddressPoolPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpConfiguration {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IpConfigurationPropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl IpConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpConfigurationPropertiesFormat {
    #[serde(rename = "privateIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "privateIPAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<ip_configuration_properties_format::PrivateIpAllocationMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
    #[serde(rename = "publicIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<PublicIpAddress>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl IpConfigurationPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ip_configuration_properties_format {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrivateIpAllocationMethod {
        Static,
        Dynamic,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundNatRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InboundNatRulePropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl InboundNatRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundNatRulePropertiesFormat {
    #[serde(rename = "frontendIPConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub frontend_ip_configuration: Option<SubResource>,
    #[serde(rename = "backendIPConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub backend_ip_configuration: Option<NetworkInterfaceIpConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<TransportProtocol>,
    #[serde(rename = "frontendPort", default, skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    #[serde(rename = "backendPort", default, skip_serializing_if = "Option::is_none")]
    pub backend_port: Option<i32>,
    #[serde(rename = "idleTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i32>,
    #[serde(rename = "enableFloatingIP", default, skip_serializing_if = "Option::is_none")]
    pub enable_floating_ip: Option<bool>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl InboundNatRulePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterface {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkInterfacePropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl NetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceDnsSettings {
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<String>,
    #[serde(rename = "appliedDnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub applied_dns_servers: Vec<String>,
    #[serde(rename = "internalDnsNameLabel", default, skip_serializing_if = "Option::is_none")]
    pub internal_dns_name_label: Option<String>,
    #[serde(rename = "internalFqdn", default, skip_serializing_if = "Option::is_none")]
    pub internal_fqdn: Option<String>,
    #[serde(rename = "internalDomainNameSuffix", default, skip_serializing_if = "Option::is_none")]
    pub internal_domain_name_suffix: Option<String>,
}
impl NetworkInterfaceDnsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceIpConfiguration {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkInterfaceIpConfigurationPropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl NetworkInterfaceIpConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceIpConfigurationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkInterfaceIpConfiguration>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl NetworkInterfaceIpConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceIpConfigurationPropertiesFormat {
    #[serde(rename = "applicationGatewayBackendAddressPools", default, skip_serializing_if = "Vec::is_empty")]
    pub application_gateway_backend_address_pools: Vec<ApplicationGatewayBackendAddressPool>,
    #[serde(rename = "loadBalancerBackendAddressPools", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_backend_address_pools: Vec<BackendAddressPool>,
    #[serde(rename = "loadBalancerInboundNatRules", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_inbound_nat_rules: Vec<InboundNatRule>,
    #[serde(rename = "privateIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "privateIPAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<network_interface_ip_configuration_properties_format::PrivateIpAllocationMethod>,
    #[serde(rename = "privateIPAddressVersion", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address_version: Option<network_interface_ip_configuration_properties_format::PrivateIpAddressVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "publicIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<PublicIpAddress>,
    #[serde(rename = "applicationSecurityGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub application_security_groups: Vec<ApplicationSecurityGroup>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl NetworkInterfaceIpConfigurationPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_interface_ip_configuration_properties_format {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrivateIpAllocationMethod {
        Static,
        Dynamic,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrivateIpAddressVersion {
        IPv4,
        IPv6,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkInterface>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl NetworkInterfaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfacePropertiesFormat {
    #[serde(rename = "virtualMachine", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<SubResource>,
    #[serde(rename = "networkSecurityGroup", default, skip_serializing_if = "Option::is_none")]
    pub network_security_group: Option<NetworkSecurityGroup>,
    #[serde(rename = "ipConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_configurations: Vec<NetworkInterfaceIpConfiguration>,
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<NetworkInterfaceDnsSettings>,
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "enableAcceleratedNetworking", default, skip_serializing_if = "Option::is_none")]
    pub enable_accelerated_networking: Option<bool>,
    #[serde(rename = "enableIPForwarding", default, skip_serializing_if = "Option::is_none")]
    pub enable_ip_forwarding: Option<bool>,
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl NetworkInterfacePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSecurityGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkSecurityGroupPropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl NetworkSecurityGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSecurityGroupPropertiesFormat {
    #[serde(rename = "securityRules", default, skip_serializing_if = "Vec::is_empty")]
    pub security_rules: Vec<SecurityRule>,
    #[serde(rename = "defaultSecurityRules", default, skip_serializing_if = "Vec::is_empty")]
    pub default_security_rules: Vec<SecurityRule>,
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterface>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<Subnet>,
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl NetworkSecurityGroupPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddress {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PublicIpAddressSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Box<Option<PublicIpAddressPropertiesFormat>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl PublicIpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddressDnsSettings {
    #[serde(rename = "domainNameLabel", default, skip_serializing_if = "Option::is_none")]
    pub domain_name_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "reverseFqdn", default, skip_serializing_if = "Option::is_none")]
    pub reverse_fqdn: Option<String>,
}
impl PublicIpAddressDnsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddressListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PublicIpAddress>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PublicIpAddressListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddressPropertiesFormat {
    #[serde(rename = "publicIPAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_allocation_method: Option<public_ip_address_properties_format::PublicIpAllocationMethod>,
    #[serde(rename = "publicIPAddressVersion", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_version: Option<public_ip_address_properties_format::PublicIpAddressVersion>,
    #[serde(rename = "ipConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub ip_configuration: Option<IpConfiguration>,
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<PublicIpAddressDnsSettings>,
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "idleTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i32>,
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl PublicIpAddressPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod public_ip_address_properties_format {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicIpAllocationMethod {
        Static,
        Dynamic,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicIpAddressVersion {
        IPv4,
        IPv6,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddressSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<public_ip_address_sku::Name>,
}
impl PublicIpAddressSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod public_ip_address_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Basic,
        Standard,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNavigationLink {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceNavigationLinkFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ResourceNavigationLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNavigationLinkFormat {
    #[serde(rename = "linkedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub linked_resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ResourceNavigationLinkFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Route {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoutePropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Route {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutePropertiesFormat {
    #[serde(rename = "addressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub address_prefix: Option<String>,
    #[serde(rename = "nextHopType")]
    pub next_hop_type: route_properties_format::NextHopType,
    #[serde(rename = "nextHopIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub next_hop_ip_address: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl RoutePropertiesFormat {
    pub fn new(next_hop_type: route_properties_format::NextHopType) -> Self {
        Self {
            address_prefix: None,
            next_hop_type,
            next_hop_ip_address: None,
            provisioning_state: None,
        }
    }
}
pub mod route_properties_format {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NextHopType {
        VirtualNetworkGateway,
        VnetLocal,
        Internet,
        VirtualAppliance,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RouteTable {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RouteTablePropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl RouteTable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RouteTablePropertiesFormat {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<Route>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<Subnet>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl RouteTablePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityRulePropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl SecurityRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityRulePropertiesFormat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub protocol: security_rule_properties_format::Protocol,
    #[serde(rename = "sourcePortRange", default, skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<String>,
    #[serde(rename = "destinationPortRange", default, skip_serializing_if = "Option::is_none")]
    pub destination_port_range: Option<String>,
    #[serde(rename = "sourceAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub source_address_prefix: Option<String>,
    #[serde(rename = "sourceAddressPrefixes", default, skip_serializing_if = "Vec::is_empty")]
    pub source_address_prefixes: Vec<String>,
    #[serde(rename = "sourceApplicationSecurityGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub source_application_security_groups: Vec<ApplicationSecurityGroup>,
    #[serde(rename = "destinationAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub destination_address_prefix: Option<String>,
    #[serde(rename = "destinationAddressPrefixes", default, skip_serializing_if = "Vec::is_empty")]
    pub destination_address_prefixes: Vec<String>,
    #[serde(rename = "destinationApplicationSecurityGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub destination_application_security_groups: Vec<ApplicationSecurityGroup>,
    #[serde(rename = "sourcePortRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub source_port_ranges: Vec<String>,
    #[serde(rename = "destinationPortRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub destination_port_ranges: Vec<String>,
    pub access: security_rule_properties_format::Access,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    pub direction: security_rule_properties_format::Direction,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl SecurityRulePropertiesFormat {
    pub fn new(
        protocol: security_rule_properties_format::Protocol,
        access: security_rule_properties_format::Access,
        direction: security_rule_properties_format::Direction,
    ) -> Self {
        Self {
            description: None,
            protocol,
            source_port_range: None,
            destination_port_range: None,
            source_address_prefix: None,
            source_address_prefixes: Vec::new(),
            source_application_security_groups: Vec::new(),
            destination_address_prefix: None,
            destination_address_prefixes: Vec::new(),
            destination_application_security_groups: Vec::new(),
            source_port_ranges: Vec::new(),
            destination_port_ranges: Vec::new(),
            access,
            priority: None,
            direction,
            provisioning_state: None,
        }
    }
}
pub mod security_rule_properties_format {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        Tcp,
        Udp,
        #[serde(rename = "*")]
        U2a,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Access {
        Allow,
        Deny,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Direction {
        Inbound,
        Outbound,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointPropertiesFormat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ServiceEndpointPropertiesFormat {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subnet {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubnetPropertiesFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Subnet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetPropertiesFormat {
    #[serde(rename = "addressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub address_prefix: Option<String>,
    #[serde(rename = "networkSecurityGroup", default, skip_serializing_if = "Option::is_none")]
    pub network_security_group: Option<NetworkSecurityGroup>,
    #[serde(rename = "routeTable", default, skip_serializing_if = "Option::is_none")]
    pub route_table: Option<RouteTable>,
    #[serde(rename = "serviceEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub service_endpoints: Vec<ServiceEndpointPropertiesFormat>,
    #[serde(rename = "ipConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_configurations: Vec<IpConfiguration>,
    #[serde(rename = "resourceNavigationLinks", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_navigation_links: Vec<ResourceNavigationLink>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl SubnetPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransportProtocol {
    Udp,
    Tcp,
    All,
}
