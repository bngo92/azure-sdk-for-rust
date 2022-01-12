#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommonExportProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<common_export_properties::Format>,
    #[serde(rename = "deliveryInfo")]
    pub delivery_info: ExportDeliveryInfo,
    pub definition: QueryDefinition,
}
impl CommonExportProperties {
    pub fn new(delivery_info: ExportDeliveryInfo, definition: QueryDefinition) -> Self {
        Self {
            format: None,
            delivery_info,
            definition,
        }
    }
}
pub mod common_export_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        Csv,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DimensionProperties>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "filterEnabled", default, skip_serializing_if = "Option::is_none")]
    pub filter_enabled: Option<bool>,
    #[serde(rename = "groupingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub grouping_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[serde(rename = "usageEnd", default, skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DimensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Dimension>,
}
impl DimensionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
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
    pub error: Option<ErrorDetails>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Export {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExportProperties>,
}
impl Export {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDeliveryDestination {
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    pub container: String,
    #[serde(rename = "rootFolderPath", default, skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<String>,
}
impl ExportDeliveryDestination {
    pub fn new(resource_id: String, container: String) -> Self {
        Self {
            resource_id,
            container,
            root_folder_path: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDeliveryInfo {
    pub destination: ExportDeliveryDestination,
}
impl ExportDeliveryInfo {
    pub fn new(destination: ExportDeliveryDestination) -> Self {
        Self { destination }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportExecution {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExportExecutionProperties>,
}
impl ExportExecution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportExecutionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExportExecution>,
}
impl ExportExecutionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportExecutionProperties {
    #[serde(rename = "executionType", default, skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<export_execution_properties::ExecutionType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<export_execution_properties::Status>,
    #[serde(rename = "submittedBy", default, skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    #[serde(rename = "submittedTime", default, skip_serializing_if = "Option::is_none")]
    pub submitted_time: Option<String>,
    #[serde(rename = "processingStartTime", default, skip_serializing_if = "Option::is_none")]
    pub processing_start_time: Option<String>,
    #[serde(rename = "processingEndTime", default, skip_serializing_if = "Option::is_none")]
    pub processing_end_time: Option<String>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "runSettings", default, skip_serializing_if = "Option::is_none")]
    pub run_settings: Option<CommonExportProperties>,
}
impl ExportExecutionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod export_execution_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExecutionType {
        OnDemand,
        Scheduled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Queued,
        InProgress,
        Completed,
        Failed,
        Timeout,
        NewDataNotAvailable,
        DataNotAvailable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Export>,
}
impl ExportListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportProperties {
    #[serde(flatten)]
    pub common_export_properties: CommonExportProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ExportSchedule>,
}
impl ExportProperties {
    pub fn new(common_export_properties: CommonExportProperties) -> Self {
        Self {
            common_export_properties,
            schedule: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportRecurrencePeriod {
    pub from: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}
impl ExportRecurrencePeriod {
    pub fn new(from: String) -> Self {
        Self { from, to: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportSchedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<export_schedule::Status>,
    pub recurrence: export_schedule::Recurrence,
    #[serde(rename = "recurrencePeriod", default, skip_serializing_if = "Option::is_none")]
    pub recurrence_period: Option<ExportRecurrencePeriod>,
}
impl ExportSchedule {
    pub fn new(recurrence: export_schedule::Recurrence) -> Self {
        Self {
            status: None,
            recurrence,
            recurrence_period: None,
        }
    }
}
pub mod export_schedule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Active,
        Inactive,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Recurrence {
        Daily,
        Weekly,
        Monthly,
        Annually,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
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
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
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
pub struct QueryAggregation {
    pub name: String,
    pub function: query_aggregation::Function,
}
impl QueryAggregation {
    pub fn new(name: String, function: query_aggregation::Function) -> Self {
        Self { name, function }
    }
}
pub mod query_aggregation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Function {
        Sum,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryColumn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl QueryColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryColumnType {
    Tag,
    Dimension,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryComparisonExpression {
    pub name: String,
    pub operator: query_comparison_expression::Operator,
    pub values: Vec<String>,
}
impl QueryComparisonExpression {
    pub fn new(name: String, operator: query_comparison_expression::Operator, values: Vec<String>) -> Self {
        Self { name, operator, values }
    }
}
pub mod query_comparison_expression {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        In,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryDataset {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<query_dataset::Granularity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<QueryDatasetConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grouping: Vec<QueryGrouping>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sorting: Vec<QuerySortingConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<QueryFilter>,
}
impl QueryDataset {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod query_dataset {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Granularity {
        Daily,
        Hourly,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryDatasetConfiguration {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<String>,
}
impl QueryDatasetConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryDefinition {
    #[serde(rename = "type")]
    pub type_: query_definition::Type,
    pub timeframe: query_definition::Timeframe,
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<QueryTimePeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataset: Option<QueryDataset>,
}
impl QueryDefinition {
    pub fn new(type_: query_definition::Type, timeframe: query_definition::Timeframe) -> Self {
        Self {
            type_,
            timeframe,
            time_period: None,
            dataset: None,
        }
    }
}
pub mod query_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Usage,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Timeframe {
        WeekToDate,
        MonthToDate,
        YearToDate,
        TheLastWeek,
        TheLastMonth,
        TheLastYear,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryFilter {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub and: Vec<QueryFilter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub or: Vec<QueryFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Box<Option<QueryFilter>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<QueryComparisonExpression>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<QueryComparisonExpression>,
}
impl QueryFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryGrouping {
    #[serde(rename = "type")]
    pub type_: QueryColumnType,
    pub name: String,
}
impl QueryGrouping {
    pub fn new(type_: QueryColumnType, name: String) -> Self {
        Self { type_, name }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryProperties {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<QueryColumn>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Vec<serde_json::Value>>,
}
impl QueryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryProperties>,
}
impl QueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuerySortingConfiguration {
    #[serde(rename = "querySortingDirection", default, skip_serializing_if = "Option::is_none")]
    pub query_sorting_direction: Option<query_sorting_configuration::QuerySortingDirection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl QuerySortingConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod query_sorting_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QuerySortingDirection {
        Ascending,
        Descending,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryTimePeriod {
    pub from: String,
    pub to: String,
}
impl QueryTimePeriod {
    pub fn new(from: String, to: String) -> Self {
        Self { from, to }
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
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
