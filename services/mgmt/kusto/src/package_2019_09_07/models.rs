#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachedDatabaseConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AttachedDatabaseConfigurationProperties>,
}
impl AttachedDatabaseConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachedDatabaseConfigurationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AttachedDatabaseConfiguration>,
}
impl AttachedDatabaseConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachedDatabaseConfigurationProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<attached_database_configuration_properties::ProvisioningState>,
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[serde(rename = "clusterResourceId")]
    pub cluster_resource_id: String,
    #[serde(rename = "attachedDatabaseNames", default, skip_serializing_if = "Vec::is_empty")]
    pub attached_database_names: Vec<String>,
    #[serde(rename = "defaultPrincipalsModificationKind")]
    pub default_principals_modification_kind: attached_database_configuration_properties::DefaultPrincipalsModificationKind,
}
impl AttachedDatabaseConfigurationProperties {
    pub fn new(
        database_name: String,
        cluster_resource_id: String,
        default_principals_modification_kind: attached_database_configuration_properties::DefaultPrincipalsModificationKind,
    ) -> Self {
        Self {
            provisioning_state: None,
            database_name,
            cluster_resource_id,
            attached_database_names: Vec::new(),
            default_principals_modification_kind,
        }
    }
}
pub mod attached_database_configuration_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Running,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Moving,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultPrincipalsModificationKind {
        Union,
        Replace,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCapacity {
    #[serde(rename = "scaleType")]
    pub scale_type: azure_capacity::ScaleType,
    pub minimum: i64,
    pub maximum: i64,
    pub default: i64,
}
impl AzureCapacity {
    pub fn new(scale_type: azure_capacity::ScaleType, minimum: i64, maximum: i64, default: i64) -> Self {
        Self {
            scale_type,
            minimum,
            maximum,
            default,
        }
    }
}
pub mod azure_capacity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScaleType {
        #[serde(rename = "automatic")]
        Automatic,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "none")]
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceSku {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AzureSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<AzureCapacity>,
}
impl AzureResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSku {
    pub name: azure_sku::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    pub tier: azure_sku::Tier,
}
impl AzureSku {
    pub fn new(name: azure_sku::Name, tier: azure_sku::Tier) -> Self {
        Self {
            name,
            capacity: None,
            tier,
        }
    }
}
pub mod azure_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_DS13_v2+1TB_PS")]
        StandardDs13V21tbPs,
        #[serde(rename = "Standard_DS13_v2+2TB_PS")]
        StandardDs13V22tbPs,
        #[serde(rename = "Standard_DS14_v2+3TB_PS")]
        StandardDs14V23tbPs,
        #[serde(rename = "Standard_DS14_v2+4TB_PS")]
        StandardDs14V24tbPs,
        #[serde(rename = "Standard_D13_v2")]
        StandardD13V2,
        #[serde(rename = "Standard_D14_v2")]
        StandardD14V2,
        #[serde(rename = "Standard_L8s")]
        StandardL8s,
        #[serde(rename = "Standard_L16s")]
        StandardL16s,
        #[serde(rename = "Standard_D11_v2")]
        StandardD11V2,
        #[serde(rename = "Standard_D12_v2")]
        StandardD12V2,
        #[serde(rename = "Standard_L4s")]
        StandardL4s,
        #[serde(rename = "Dev(No SLA)_Standard_D11_v2")]
        DevNoSlaStandardD11V2,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: check_name_request::Type,
}
impl CheckNameRequest {
    pub fn new(name: String, type_: check_name_request::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod check_name_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Kusto/clusters/databases")]
        MicrosoftKustoClustersDatabases,
        #[serde(rename = "Microsoft.Kusto/clusters/attachedDatabaseConfigurations")]
        MicrosoftKustoClustersAttachedDatabaseConfigurations,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_result::Reason>,
}
impl CheckNameResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub sku: AzureSku,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Zones>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
impl Cluster {
    pub fn new(tracked_resource: TrackedResource, sku: AzureSku) -> Self {
        Self {
            tracked_resource,
            sku,
            zones: None,
            identity: None,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterCheckNameRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: cluster_check_name_request::Type,
}
impl ClusterCheckNameRequest {
    pub fn new(name: String, type_: cluster_check_name_request::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod cluster_check_name_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Kusto/clusters")]
        MicrosoftKustoClusters,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
}
impl ClusterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<cluster_properties::State>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cluster_properties::ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "dataIngestionUri", default, skip_serializing_if = "Option::is_none")]
    pub data_ingestion_uri: Option<String>,
    #[serde(rename = "trustedExternalTenants", default, skip_serializing_if = "Vec::is_empty")]
    pub trusted_external_tenants: Vec<TrustedExternalTenant>,
    #[serde(rename = "optimizedAutoscale", default, skip_serializing_if = "Option::is_none")]
    pub optimized_autoscale: Option<OptimizedAutoscale>,
    #[serde(rename = "enableDiskEncryption", default, skip_serializing_if = "Option::is_none")]
    pub enable_disk_encryption: Option<bool>,
    #[serde(rename = "enableStreamingIngest", default, skip_serializing_if = "Option::is_none")]
    pub enable_streaming_ingest: Option<bool>,
    #[serde(rename = "virtualNetworkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_configuration: Option<VirtualNetworkConfiguration>,
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
}
impl ClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Creating,
        Unavailable,
        Running,
        Deleting,
        Deleted,
        Stopping,
        Stopped,
        Starting,
        Updating,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Running,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Moving,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdate {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AzureSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
impl ClusterUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub kind: data_connection::Kind,
}
impl DataConnection {
    pub fn new(kind: data_connection::Kind) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            location: None,
            kind,
        }
    }
}
pub mod data_connection {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        EventHub,
        EventGrid,
        IotHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectionCheckNameRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: data_connection_check_name_request::Type,
}
impl DataConnectionCheckNameRequest {
    pub fn new(name: String, type_: data_connection_check_name_request::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod data_connection_check_name_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Kusto/clusters/databases/dataConnections")]
        MicrosoftKustoClustersDatabasesDataConnections,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataConnection>,
}
impl DataConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectionValidation {
    #[serde(rename = "dataConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub data_connection_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataConnection>,
}
impl DataConnectionValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectionValidationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataConnectionValidationResult>,
}
impl DataConnectionValidationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectionValidationResult {
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl DataConnectionValidationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataFormat {
    #[serde(rename = "MULTIJSON")]
    Multijson,
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "CSV")]
    Csv,
    #[serde(rename = "TSV")]
    Tsv,
    #[serde(rename = "SCSV")]
    Scsv,
    #[serde(rename = "SOHSV")]
    Sohsv,
    #[serde(rename = "PSV")]
    Psv,
    #[serde(rename = "TXT")]
    Txt,
    #[serde(rename = "RAW")]
    Raw,
    #[serde(rename = "SINGLEJSON")]
    Singlejson,
    #[serde(rename = "AVRO")]
    Avro,
    #[serde(rename = "TSVE")]
    Tsve,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Database {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<database::Kind>,
}
impl Database {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod database {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        ReadWrite,
        ReadOnlyFollowing,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,
}
impl DatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabasePrincipal {
    pub role: database_principal::Role,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: database_principal::Type,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "tenantName", default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}
impl DatabasePrincipal {
    pub fn new(role: database_principal::Role, name: String, type_: database_principal::Type) -> Self {
        Self {
            role,
            name,
            type_,
            fqn: None,
            email: None,
            app_id: None,
            tenant_name: None,
        }
    }
}
pub mod database_principal {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        Admin,
        Ingestor,
        Monitor,
        User,
        UnrestrictedViewers,
        Viewer,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        App,
        Group,
        User,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabasePrincipalListRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabasePrincipal>,
}
impl DatabasePrincipalListRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabasePrincipalListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabasePrincipal>,
}
impl DatabasePrincipalListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseStatistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
}
impl DatabaseStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGridConnectionProperties {
    #[serde(rename = "storageAccountResourceId")]
    pub storage_account_resource_id: String,
    #[serde(rename = "eventHubResourceId")]
    pub event_hub_resource_id: String,
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,
    #[serde(rename = "tableName")]
    pub table_name: String,
    #[serde(rename = "mappingRuleName", default, skip_serializing_if = "Option::is_none")]
    pub mapping_rule_name: Option<String>,
    #[serde(rename = "dataFormat")]
    pub data_format: DataFormat,
}
impl EventGridConnectionProperties {
    pub fn new(
        storage_account_resource_id: String,
        event_hub_resource_id: String,
        consumer_group: String,
        table_name: String,
        data_format: DataFormat,
    ) -> Self {
        Self {
            storage_account_resource_id,
            event_hub_resource_id,
            consumer_group,
            table_name,
            mapping_rule_name: None,
            data_format,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGridDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventGridConnectionProperties>,
}
impl EventGridDataConnection {
    pub fn new(data_connection: DataConnection) -> Self {
        Self {
            data_connection,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubConnectionProperties {
    #[serde(rename = "eventHubResourceId")]
    pub event_hub_resource_id: String,
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "mappingRuleName", default, skip_serializing_if = "Option::is_none")]
    pub mapping_rule_name: Option<String>,
    #[serde(rename = "dataFormat", default, skip_serializing_if = "Option::is_none")]
    pub data_format: Option<DataFormat>,
    #[serde(rename = "eventSystemProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub event_system_properties: Vec<String>,
}
impl EventHubConnectionProperties {
    pub fn new(event_hub_resource_id: String, consumer_group: String) -> Self {
        Self {
            event_hub_resource_id,
            consumer_group,
            table_name: None,
            mapping_rule_name: None,
            data_format: None,
            event_system_properties: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubConnectionProperties>,
}
impl EventHubDataConnection {
    pub fn new(data_connection: DataConnection) -> Self {
        Self {
            data_connection,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FollowerDatabaseDefinition {
    #[serde(rename = "clusterResourceId")]
    pub cluster_resource_id: String,
    #[serde(rename = "attachedDatabaseConfigurationName")]
    pub attached_database_configuration_name: String,
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
}
impl FollowerDatabaseDefinition {
    pub fn new(cluster_resource_id: String, attached_database_configuration_name: String) -> Self {
        Self {
            cluster_resource_id,
            attached_database_configuration_name,
            database_name: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FollowerDatabaseListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FollowerDatabaseDefinition>,
}
impl FollowerDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: identity::Type,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl Identity {
    pub fn new(type_: identity::Type) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        None,
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubConnectionProperties {
    #[serde(rename = "iotHubResourceId")]
    pub iot_hub_resource_id: String,
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "mappingRuleName", default, skip_serializing_if = "Option::is_none")]
    pub mapping_rule_name: Option<String>,
    #[serde(rename = "dataFormat", default, skip_serializing_if = "Option::is_none")]
    pub data_format: Option<DataFormat>,
    #[serde(rename = "eventSystemProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub event_system_properties: Vec<String>,
    #[serde(rename = "sharedAccessPolicyName")]
    pub shared_access_policy_name: String,
}
impl IotHubConnectionProperties {
    pub fn new(iot_hub_resource_id: String, consumer_group: String, shared_access_policy_name: String) -> Self {
        Self {
            iot_hub_resource_id,
            consumer_group,
            table_name: None,
            mapping_rule_name: None,
            data_format: None,
            event_system_properties: Vec::new(),
            shared_access_policy_name,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IotHubConnectionProperties>,
}
impl IotHubDataConnection {
    pub fn new(data_connection: DataConnection) -> Self {
        Self {
            data_connection,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[serde(rename = "keyVersion")]
    pub key_version: String,
    #[serde(rename = "keyVaultUri")]
    pub key_vault_uri: String,
}
impl KeyVaultProperties {
    pub fn new(key_name: String, key_version: String, key_vault_uri: String) -> Self {
        Self {
            key_name,
            key_version,
            key_vault_uri,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListResourceSkusResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AzureResourceSku>,
}
impl ListResourceSkusResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptimizedAutoscale {
    pub version: i64,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    pub minimum: i64,
    pub maximum: i64,
}
impl OptimizedAutoscale {
    pub fn new(version: i64, is_enabled: bool, minimum: i64, maximum: i64) -> Self {
        Self {
            version,
            is_enabled,
            minimum,
            maximum,
        }
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
pub struct ReadOnlyFollowingDatabase {
    #[serde(flatten)]
    pub database: Database,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReadOnlyFollowingDatabaseProperties>,
}
impl ReadOnlyFollowingDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReadOnlyFollowingDatabaseProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<read_only_following_database_properties::ProvisioningState>,
    #[serde(rename = "softDeletePeriod", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_period: Option<String>,
    #[serde(rename = "hotCachePeriod", default, skip_serializing_if = "Option::is_none")]
    pub hot_cache_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DatabaseStatistics>,
    #[serde(rename = "leaderClusterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub leader_cluster_resource_id: Option<String>,
    #[serde(rename = "attachedDatabaseConfigurationName", default, skip_serializing_if = "Option::is_none")]
    pub attached_database_configuration_name: Option<String>,
    #[serde(rename = "principalsModificationKind", default, skip_serializing_if = "Option::is_none")]
    pub principals_modification_kind: Option<read_only_following_database_properties::PrincipalsModificationKind>,
}
impl ReadOnlyFollowingDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod read_only_following_database_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Running,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Moving,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrincipalsModificationKind {
        Union,
        Replace,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReadWriteDatabase {
    #[serde(flatten)]
    pub database: Database,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReadWriteDatabaseProperties>,
}
impl ReadWriteDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReadWriteDatabaseProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<read_write_database_properties::ProvisioningState>,
    #[serde(rename = "softDeletePeriod", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_period: Option<String>,
    #[serde(rename = "hotCachePeriod", default, skip_serializing_if = "Option::is_none")]
    pub hot_cache_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DatabaseStatistics>,
}
impl ReadWriteDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod read_write_database_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Running,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Moving,
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
pub struct SkuDescription {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<SkuLocationInfoItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<serde_json::Value>,
}
impl SkuDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuDescription>,
}
impl SkuDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuLocationInfoItem {
    pub location: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl SkuLocationInfoItem {
    pub fn new(location: String) -> Self {
        Self {
            location,
            zones: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrustedExternalTenant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl TrustedExternalTenant {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkConfiguration {
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[serde(rename = "enginePublicIpId")]
    pub engine_public_ip_id: String,
    #[serde(rename = "dataManagementPublicIpId")]
    pub data_management_public_ip_id: String,
}
impl VirtualNetworkConfiguration {
    pub fn new(subnet_id: String, engine_public_ip_id: String, data_management_public_ip_id: String) -> Self {
        Self {
            subnet_id,
            engine_public_ip_id,
            data_management_public_ip_id,
        }
    }
}
pub type Zones = Vec<String>;
