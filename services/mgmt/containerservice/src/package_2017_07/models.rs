#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerService {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContainerServiceProperties>,
}
impl ContainerService {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceAgentPoolProfile {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "vmSize")]
    pub vm_size: ContainerServiceVmSize,
    #[serde(rename = "osDiskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_size_gb: Option<ContainerServiceOsDisk>,
    #[serde(rename = "dnsPrefix", default, skip_serializing_if = "Option::is_none")]
    pub dns_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<i64>,
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<ContainerServiceStorageProfile>,
    #[serde(rename = "vnetSubnetID", default, skip_serializing_if = "Option::is_none")]
    pub vnet_subnet_id: Option<ContainerServiceVnetSubnetId>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
}
impl ContainerServiceAgentPoolProfile {
    pub fn new(name: String, vm_size: ContainerServiceVmSize) -> Self {
        Self {
            name,
            count: None,
            vm_size,
            os_disk_size_gb: None,
            dns_prefix: None,
            fqdn: None,
            ports: Vec::new(),
            storage_profile: None,
            vnet_subnet_id: None,
            os_type: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceCustomProfile {
    pub orchestrator: String,
}
impl ContainerServiceCustomProfile {
    pub fn new(orchestrator: String) -> Self {
        Self { orchestrator }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceDiagnosticsProfile {
    #[serde(rename = "vmDiagnostics")]
    pub vm_diagnostics: ContainerServiceVmDiagnostics,
}
impl ContainerServiceDiagnosticsProfile {
    pub fn new(vm_diagnostics: ContainerServiceVmDiagnostics) -> Self {
        Self { vm_diagnostics }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceLinuxProfile {
    #[serde(rename = "adminUsername")]
    pub admin_username: String,
    pub ssh: ContainerServiceSshConfiguration,
}
impl ContainerServiceLinuxProfile {
    pub fn new(admin_username: String, ssh: ContainerServiceSshConfiguration) -> Self {
        Self { admin_username, ssh }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ContainerService>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ContainerServiceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceMasterProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<container_service_master_profile::Count>,
    #[serde(rename = "dnsPrefix")]
    pub dns_prefix: String,
    #[serde(rename = "vmSize")]
    pub vm_size: ContainerServiceVmSize,
    #[serde(rename = "osDiskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_size_gb: Option<ContainerServiceOsDisk>,
    #[serde(rename = "vnetSubnetID", default, skip_serializing_if = "Option::is_none")]
    pub vnet_subnet_id: Option<ContainerServiceVnetSubnetId>,
    #[serde(rename = "firstConsecutiveStaticIP", default, skip_serializing_if = "Option::is_none")]
    pub first_consecutive_static_ip: Option<String>,
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<ContainerServiceStorageProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
}
impl ContainerServiceMasterProfile {
    pub fn new(dns_prefix: String, vm_size: ContainerServiceVmSize) -> Self {
        Self {
            count: None,
            dns_prefix,
            vm_size,
            os_disk_size_gb: None,
            vnet_subnet_id: None,
            first_consecutive_static_ip: None,
            storage_profile: None,
            fqdn: None,
        }
    }
}
pub mod container_service_master_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Count {}
}
pub type ContainerServiceOsDisk = i32;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceOrchestratorProfile {
    #[serde(rename = "orchestratorType")]
    pub orchestrator_type: container_service_orchestrator_profile::OrchestratorType,
    #[serde(rename = "orchestratorVersion", default, skip_serializing_if = "Option::is_none")]
    pub orchestrator_version: Option<String>,
}
impl ContainerServiceOrchestratorProfile {
    pub fn new(orchestrator_type: container_service_orchestrator_profile::OrchestratorType) -> Self {
        Self {
            orchestrator_type,
            orchestrator_version: None,
        }
    }
}
pub mod container_service_orchestrator_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OrchestratorType {
        Kubernetes,
        Swarm,
        #[serde(rename = "DCOS")]
        Dcos,
        #[serde(rename = "DockerCE")]
        DockerCe,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "orchestratorProfile")]
    pub orchestrator_profile: ContainerServiceOrchestratorProfile,
    #[serde(rename = "customProfile", default, skip_serializing_if = "Option::is_none")]
    pub custom_profile: Option<ContainerServiceCustomProfile>,
    #[serde(rename = "servicePrincipalProfile", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_profile: Option<ContainerServiceServicePrincipalProfile>,
    #[serde(rename = "masterProfile")]
    pub master_profile: ContainerServiceMasterProfile,
    #[serde(rename = "agentPoolProfiles", default, skip_serializing_if = "Vec::is_empty")]
    pub agent_pool_profiles: Vec<ContainerServiceAgentPoolProfile>,
    #[serde(rename = "windowsProfile", default, skip_serializing_if = "Option::is_none")]
    pub windows_profile: Option<ContainerServiceWindowsProfile>,
    #[serde(rename = "linuxProfile")]
    pub linux_profile: ContainerServiceLinuxProfile,
    #[serde(rename = "diagnosticsProfile", default, skip_serializing_if = "Option::is_none")]
    pub diagnostics_profile: Option<ContainerServiceDiagnosticsProfile>,
}
impl ContainerServiceProperties {
    pub fn new(
        orchestrator_profile: ContainerServiceOrchestratorProfile,
        master_profile: ContainerServiceMasterProfile,
        linux_profile: ContainerServiceLinuxProfile,
    ) -> Self {
        Self {
            provisioning_state: None,
            orchestrator_profile,
            custom_profile: None,
            service_principal_profile: None,
            master_profile,
            agent_pool_profiles: Vec::new(),
            windows_profile: None,
            linux_profile,
            diagnostics_profile: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceServicePrincipalProfile {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "keyVaultSecretRef", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_secret_ref: Option<KeyVaultSecretRef>,
}
impl ContainerServiceServicePrincipalProfile {
    pub fn new(client_id: String) -> Self {
        Self {
            client_id,
            secret: None,
            key_vault_secret_ref: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceSshConfiguration {
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<ContainerServiceSshPublicKey>,
}
impl ContainerServiceSshConfiguration {
    pub fn new(public_keys: Vec<ContainerServiceSshPublicKey>) -> Self {
        Self { public_keys }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceSshPublicKey {
    #[serde(rename = "keyData")]
    pub key_data: String,
}
impl ContainerServiceSshPublicKey {
    pub fn new(key_data: String) -> Self {
        Self { key_data }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContainerServiceStorageProfile {
    StorageAccount,
    ManagedDisks,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceVmDiagnostics {
    pub enabled: bool,
    #[serde(rename = "storageUri", default, skip_serializing_if = "Option::is_none")]
    pub storage_uri: Option<String>,
}
impl ContainerServiceVmDiagnostics {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            storage_uri: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContainerServiceVmSize {
    #[serde(rename = "Standard_A1")]
    StandardA1,
    #[serde(rename = "Standard_A10")]
    StandardA10,
    #[serde(rename = "Standard_A11")]
    StandardA11,
    #[serde(rename = "Standard_A1_v2")]
    StandardA1V2,
    #[serde(rename = "Standard_A2")]
    StandardA2,
    #[serde(rename = "Standard_A2_v2")]
    StandardA2V2,
    #[serde(rename = "Standard_A2m_v2")]
    StandardA2mV2,
    #[serde(rename = "Standard_A3")]
    StandardA3,
    #[serde(rename = "Standard_A4")]
    StandardA4,
    #[serde(rename = "Standard_A4_v2")]
    StandardA4V2,
    #[serde(rename = "Standard_A4m_v2")]
    StandardA4mV2,
    #[serde(rename = "Standard_A5")]
    StandardA5,
    #[serde(rename = "Standard_A6")]
    StandardA6,
    #[serde(rename = "Standard_A7")]
    StandardA7,
    #[serde(rename = "Standard_A8")]
    StandardA8,
    #[serde(rename = "Standard_A8_v2")]
    StandardA8V2,
    #[serde(rename = "Standard_A8m_v2")]
    StandardA8mV2,
    #[serde(rename = "Standard_A9")]
    StandardA9,
    #[serde(rename = "Standard_B2ms")]
    StandardB2ms,
    #[serde(rename = "Standard_B2s")]
    StandardB2s,
    #[serde(rename = "Standard_B4ms")]
    StandardB4ms,
    #[serde(rename = "Standard_B8ms")]
    StandardB8ms,
    #[serde(rename = "Standard_D1")]
    StandardD1,
    #[serde(rename = "Standard_D11")]
    StandardD11,
    #[serde(rename = "Standard_D11_v2")]
    StandardD11V2,
    #[serde(rename = "Standard_D11_v2_Promo")]
    StandardD11V2Promo,
    #[serde(rename = "Standard_D12")]
    StandardD12,
    #[serde(rename = "Standard_D12_v2")]
    StandardD12V2,
    #[serde(rename = "Standard_D12_v2_Promo")]
    StandardD12V2Promo,
    #[serde(rename = "Standard_D13")]
    StandardD13,
    #[serde(rename = "Standard_D13_v2")]
    StandardD13V2,
    #[serde(rename = "Standard_D13_v2_Promo")]
    StandardD13V2Promo,
    #[serde(rename = "Standard_D14")]
    StandardD14,
    #[serde(rename = "Standard_D14_v2")]
    StandardD14V2,
    #[serde(rename = "Standard_D14_v2_Promo")]
    StandardD14V2Promo,
    #[serde(rename = "Standard_D15_v2")]
    StandardD15V2,
    #[serde(rename = "Standard_D16_v3")]
    StandardD16V3,
    #[serde(rename = "Standard_D16s_v3")]
    StandardD16sV3,
    #[serde(rename = "Standard_D1_v2")]
    StandardD1V2,
    #[serde(rename = "Standard_D2")]
    StandardD2,
    #[serde(rename = "Standard_D2_v2")]
    StandardD2V2,
    #[serde(rename = "Standard_D2_v2_Promo")]
    StandardD2V2Promo,
    #[serde(rename = "Standard_D2_v3")]
    StandardD2V3,
    #[serde(rename = "Standard_D2s_v3")]
    StandardD2sV3,
    #[serde(rename = "Standard_D3")]
    StandardD3,
    #[serde(rename = "Standard_D32_v3")]
    StandardD32V3,
    #[serde(rename = "Standard_D32s_v3")]
    StandardD32sV3,
    #[serde(rename = "Standard_D3_v2")]
    StandardD3V2,
    #[serde(rename = "Standard_D3_v2_Promo")]
    StandardD3V2Promo,
    #[serde(rename = "Standard_D4")]
    StandardD4,
    #[serde(rename = "Standard_D4_v2")]
    StandardD4V2,
    #[serde(rename = "Standard_D4_v2_Promo")]
    StandardD4V2Promo,
    #[serde(rename = "Standard_D4_v3")]
    StandardD4V3,
    #[serde(rename = "Standard_D4s_v3")]
    StandardD4sV3,
    #[serde(rename = "Standard_D5_v2")]
    StandardD5V2,
    #[serde(rename = "Standard_D5_v2_Promo")]
    StandardD5V2Promo,
    #[serde(rename = "Standard_D64_v3")]
    StandardD64V3,
    #[serde(rename = "Standard_D64s_v3")]
    StandardD64sV3,
    #[serde(rename = "Standard_D8_v3")]
    StandardD8V3,
    #[serde(rename = "Standard_D8s_v3")]
    StandardD8sV3,
    #[serde(rename = "Standard_DS1")]
    StandardDs1,
    #[serde(rename = "Standard_DS11")]
    StandardDs11,
    #[serde(rename = "Standard_DS11_v2")]
    StandardDs11V2,
    #[serde(rename = "Standard_DS11_v2_Promo")]
    StandardDs11V2Promo,
    #[serde(rename = "Standard_DS12")]
    StandardDs12,
    #[serde(rename = "Standard_DS12_v2")]
    StandardDs12V2,
    #[serde(rename = "Standard_DS12_v2_Promo")]
    StandardDs12V2Promo,
    #[serde(rename = "Standard_DS13")]
    StandardDs13,
    #[serde(rename = "Standard_DS13-2_v2")]
    StandardDs132V2,
    #[serde(rename = "Standard_DS13-4_v2")]
    StandardDs134V2,
    #[serde(rename = "Standard_DS13_v2")]
    StandardDs13V2,
    #[serde(rename = "Standard_DS13_v2_Promo")]
    StandardDs13V2Promo,
    #[serde(rename = "Standard_DS14")]
    StandardDs14,
    #[serde(rename = "Standard_DS14-4_v2")]
    StandardDs144V2,
    #[serde(rename = "Standard_DS14-8_v2")]
    StandardDs148V2,
    #[serde(rename = "Standard_DS14_v2")]
    StandardDs14V2,
    #[serde(rename = "Standard_DS14_v2_Promo")]
    StandardDs14V2Promo,
    #[serde(rename = "Standard_DS15_v2")]
    StandardDs15V2,
    #[serde(rename = "Standard_DS1_v2")]
    StandardDs1V2,
    #[serde(rename = "Standard_DS2")]
    StandardDs2,
    #[serde(rename = "Standard_DS2_v2")]
    StandardDs2V2,
    #[serde(rename = "Standard_DS2_v2_Promo")]
    StandardDs2V2Promo,
    #[serde(rename = "Standard_DS3")]
    StandardDs3,
    #[serde(rename = "Standard_DS3_v2")]
    StandardDs3V2,
    #[serde(rename = "Standard_DS3_v2_Promo")]
    StandardDs3V2Promo,
    #[serde(rename = "Standard_DS4")]
    StandardDs4,
    #[serde(rename = "Standard_DS4_v2")]
    StandardDs4V2,
    #[serde(rename = "Standard_DS4_v2_Promo")]
    StandardDs4V2Promo,
    #[serde(rename = "Standard_DS5_v2")]
    StandardDs5V2,
    #[serde(rename = "Standard_DS5_v2_Promo")]
    StandardDs5V2Promo,
    #[serde(rename = "Standard_E16_v3")]
    StandardE16V3,
    #[serde(rename = "Standard_E16s_v3")]
    StandardE16sV3,
    #[serde(rename = "Standard_E2_v3")]
    StandardE2V3,
    #[serde(rename = "Standard_E2s_v3")]
    StandardE2sV3,
    #[serde(rename = "Standard_E32-16s_v3")]
    StandardE3216sV3,
    #[serde(rename = "Standard_E32-8s_v3")]
    StandardE328sV3,
    #[serde(rename = "Standard_E32_v3")]
    StandardE32V3,
    #[serde(rename = "Standard_E32s_v3")]
    StandardE32sV3,
    #[serde(rename = "Standard_E4_v3")]
    StandardE4V3,
    #[serde(rename = "Standard_E4s_v3")]
    StandardE4sV3,
    #[serde(rename = "Standard_E64-16s_v3")]
    StandardE6416sV3,
    #[serde(rename = "Standard_E64-32s_v3")]
    StandardE6432sV3,
    #[serde(rename = "Standard_E64_v3")]
    StandardE64V3,
    #[serde(rename = "Standard_E64s_v3")]
    StandardE64sV3,
    #[serde(rename = "Standard_E8_v3")]
    StandardE8V3,
    #[serde(rename = "Standard_E8s_v3")]
    StandardE8sV3,
    #[serde(rename = "Standard_F1")]
    StandardF1,
    #[serde(rename = "Standard_F16")]
    StandardF16,
    #[serde(rename = "Standard_F16s")]
    StandardF16s,
    #[serde(rename = "Standard_F16s_v2")]
    StandardF16sV2,
    #[serde(rename = "Standard_F1s")]
    StandardF1s,
    #[serde(rename = "Standard_F2")]
    StandardF2,
    #[serde(rename = "Standard_F2s")]
    StandardF2s,
    #[serde(rename = "Standard_F2s_v2")]
    StandardF2sV2,
    #[serde(rename = "Standard_F32s_v2")]
    StandardF32sV2,
    #[serde(rename = "Standard_F4")]
    StandardF4,
    #[serde(rename = "Standard_F4s")]
    StandardF4s,
    #[serde(rename = "Standard_F4s_v2")]
    StandardF4sV2,
    #[serde(rename = "Standard_F64s_v2")]
    StandardF64sV2,
    #[serde(rename = "Standard_F72s_v2")]
    StandardF72sV2,
    #[serde(rename = "Standard_F8")]
    StandardF8,
    #[serde(rename = "Standard_F8s")]
    StandardF8s,
    #[serde(rename = "Standard_F8s_v2")]
    StandardF8sV2,
    #[serde(rename = "Standard_G1")]
    StandardG1,
    #[serde(rename = "Standard_G2")]
    StandardG2,
    #[serde(rename = "Standard_G3")]
    StandardG3,
    #[serde(rename = "Standard_G4")]
    StandardG4,
    #[serde(rename = "Standard_G5")]
    StandardG5,
    #[serde(rename = "Standard_GS1")]
    StandardGs1,
    #[serde(rename = "Standard_GS2")]
    StandardGs2,
    #[serde(rename = "Standard_GS3")]
    StandardGs3,
    #[serde(rename = "Standard_GS4")]
    StandardGs4,
    #[serde(rename = "Standard_GS4-4")]
    StandardGs44,
    #[serde(rename = "Standard_GS4-8")]
    StandardGs48,
    #[serde(rename = "Standard_GS5")]
    StandardGs5,
    #[serde(rename = "Standard_GS5-16")]
    StandardGs516,
    #[serde(rename = "Standard_GS5-8")]
    StandardGs58,
    #[serde(rename = "Standard_H16")]
    StandardH16,
    #[serde(rename = "Standard_H16m")]
    StandardH16m,
    #[serde(rename = "Standard_H16mr")]
    StandardH16mr,
    #[serde(rename = "Standard_H16r")]
    StandardH16r,
    #[serde(rename = "Standard_H8")]
    StandardH8,
    #[serde(rename = "Standard_H8m")]
    StandardH8m,
    #[serde(rename = "Standard_L16s")]
    StandardL16s,
    #[serde(rename = "Standard_L32s")]
    StandardL32s,
    #[serde(rename = "Standard_L4s")]
    StandardL4s,
    #[serde(rename = "Standard_L8s")]
    StandardL8s,
    #[serde(rename = "Standard_M128-32ms")]
    StandardM12832ms,
    #[serde(rename = "Standard_M128-64ms")]
    StandardM12864ms,
    #[serde(rename = "Standard_M128ms")]
    StandardM128ms,
    #[serde(rename = "Standard_M128s")]
    StandardM128s,
    #[serde(rename = "Standard_M64-16ms")]
    StandardM6416ms,
    #[serde(rename = "Standard_M64-32ms")]
    StandardM6432ms,
    #[serde(rename = "Standard_M64ms")]
    StandardM64ms,
    #[serde(rename = "Standard_M64s")]
    StandardM64s,
    #[serde(rename = "Standard_NC12")]
    StandardNc12,
    #[serde(rename = "Standard_NC12s_v2")]
    StandardNc12sV2,
    #[serde(rename = "Standard_NC12s_v3")]
    StandardNc12sV3,
    #[serde(rename = "Standard_NC24")]
    StandardNc24,
    #[serde(rename = "Standard_NC24r")]
    StandardNc24r,
    #[serde(rename = "Standard_NC24rs_v2")]
    StandardNc24rsV2,
    #[serde(rename = "Standard_NC24rs_v3")]
    StandardNc24rsV3,
    #[serde(rename = "Standard_NC24s_v2")]
    StandardNc24sV2,
    #[serde(rename = "Standard_NC24s_v3")]
    StandardNc24sV3,
    #[serde(rename = "Standard_NC6")]
    StandardNc6,
    #[serde(rename = "Standard_NC6s_v2")]
    StandardNc6sV2,
    #[serde(rename = "Standard_NC6s_v3")]
    StandardNc6sV3,
    #[serde(rename = "Standard_ND12s")]
    StandardNd12s,
    #[serde(rename = "Standard_ND24rs")]
    StandardNd24rs,
    #[serde(rename = "Standard_ND24s")]
    StandardNd24s,
    #[serde(rename = "Standard_ND6s")]
    StandardNd6s,
    #[serde(rename = "Standard_NV12")]
    StandardNv12,
    #[serde(rename = "Standard_NV24")]
    StandardNv24,
    #[serde(rename = "Standard_NV6")]
    StandardNv6,
}
pub type ContainerServiceVnetSubnetId = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceWindowsProfile {
    #[serde(rename = "adminUsername")]
    pub admin_username: String,
    #[serde(rename = "adminPassword")]
    pub admin_password: String,
}
impl ContainerServiceWindowsProfile {
    pub fn new(admin_username: String, admin_password: String) -> Self {
        Self {
            admin_username,
            admin_password,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultSecretRef {
    #[serde(rename = "vaultID")]
    pub vault_id: String,
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl KeyVaultSecretRef {
    pub fn new(vault_id: String, secret_name: String) -> Self {
        Self {
            vault_id,
            secret_name,
            version: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OsType {
    Linux,
    Windows,
}
impl Default for OsType {
    fn default() -> Self {
        Self::Linux
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
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
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
