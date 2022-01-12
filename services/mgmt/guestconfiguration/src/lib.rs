#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2021-01-25")]
pub mod package_2021_01_25;
#[cfg(all(feature = "package-2021-01-25", not(feature = "no-default-version")))]
pub use package_2021_01_25::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-06-25")]
pub mod package_2020_06_25;
#[cfg(all(feature = "package-2020-06-25", not(feature = "no-default-version")))]
pub use package_2020_06_25::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-11-20")]
pub mod package_2018_11_20;
#[cfg(all(feature = "package-2018-11-20", not(feature = "no-default-version")))]
pub use package_2018_11_20::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-06-30-preview")]
pub mod package_2018_06_30_preview;
#[cfg(all(feature = "package-2018-06-30-preview", not(feature = "no-default-version")))]
pub use package_2018_06_30_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-01-20-preview")]
pub mod package_2018_01_20_preview;
#[cfg(all(feature = "package-2018-01-20-preview", not(feature = "no-default-version")))]
pub use package_2018_01_20_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
