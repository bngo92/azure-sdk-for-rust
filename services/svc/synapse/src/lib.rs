#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-artifacts-composite-v3")]
pub mod package_artifacts_composite_v3;
#[cfg(all(feature = "package-artifacts-composite-v3", not(feature = "no-default-version")))]
pub use package_artifacts_composite_v3::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-artifacts-composite-v2")]
pub mod package_artifacts_composite_v2;
#[cfg(all(feature = "package-artifacts-composite-v2", not(feature = "no-default-version")))]
pub use package_artifacts_composite_v2::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-artifacts-composite-v1")]
pub mod package_artifacts_composite_v1;
#[cfg(all(feature = "package-artifacts-composite-v1", not(feature = "no-default-version")))]
pub use package_artifacts_composite_v1::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-artifacts-2021-06-01-preview")]
pub mod package_artifacts_2021_06_01_preview;
#[cfg(all(feature = "package-artifacts-2021-06-01-preview", not(feature = "no-default-version")))]
pub use package_artifacts_2021_06_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-vnet-2021-06-01-preview")]
pub mod package_vnet_2021_06_01_preview;
#[cfg(all(feature = "package-vnet-2021-06-01-preview", not(feature = "no-default-version")))]
pub use package_vnet_2021_06_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-kql-script-2021-06-preview")]
pub mod package_kql_script_2021_06_preview;
#[cfg(all(feature = "package-kql-script-2021-06-preview", not(feature = "no-default-version")))]
pub use package_kql_script_2021_06_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-artifacts-2020-12-01")]
pub mod package_artifacts_2020_12_01;
#[cfg(all(feature = "package-artifacts-2020-12-01", not(feature = "no-default-version")))]
pub use package_artifacts_2020_12_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-monitoring-2020-12-01")]
pub mod package_monitoring_2020_12_01;
#[cfg(all(feature = "package-monitoring-2020-12-01", not(feature = "no-default-version")))]
pub use package_monitoring_2020_12_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-access-control-2020-12-01")]
pub mod package_access_control_2020_12_01;
#[cfg(all(feature = "package-access-control-2020-12-01", not(feature = "no-default-version")))]
pub use package_access_control_2020_12_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-vnet-2020-12-01")]
pub mod package_vnet_2020_12_01;
#[cfg(all(feature = "package-vnet-2020-12-01", not(feature = "no-default-version")))]
pub use package_vnet_2020_12_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-spark-2020-12-01")]
pub mod package_spark_2020_12_01;
#[cfg(all(feature = "package-spark-2020-12-01", not(feature = "no-default-version")))]
pub use package_spark_2020_12_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-spark-2019-11-01-preview")]
pub mod package_spark_2019_11_01_preview;
#[cfg(all(feature = "package-spark-2019-11-01-preview", not(feature = "no-default-version")))]
pub use package_spark_2019_11_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-artifacts-2019-06-01-preview")]
pub mod package_artifacts_2019_06_01_preview;
#[cfg(all(feature = "package-artifacts-2019-06-01-preview", not(feature = "no-default-version")))]
pub use package_artifacts_2019_06_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-access-control-2020-02-01-preview")]
pub mod package_access_control_2020_02_01_preview;
#[cfg(all(feature = "package-access-control-2020-02-01-preview", not(feature = "no-default-version")))]
pub use package_access_control_2020_02_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-access-control-2020-08-01-preview")]
pub mod package_access_control_2020_08_01_preview;
#[cfg(all(feature = "package-access-control-2020-08-01-preview", not(feature = "no-default-version")))]
pub use package_access_control_2020_08_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-vnet-2019-06-01-preview")]
pub mod package_vnet_2019_06_01_preview;
#[cfg(all(feature = "package-vnet-2019-06-01-preview", not(feature = "no-default-version")))]
pub use package_vnet_2019_06_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-monitoring-2019-11-01-preview")]
pub mod package_monitoring_2019_11_01_preview;
#[cfg(all(feature = "package-monitoring-2019-11-01-preview", not(feature = "no-default-version")))]
pub use package_monitoring_2019_11_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
