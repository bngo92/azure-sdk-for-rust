#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2015-11-preview")]
pub mod package_2015_11_preview;
#[cfg(all(feature = "package-2015-11-preview", not(feature = "no-default-version")))]
pub use package_2015_11_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2015-03")]
pub mod package_2015_03;
#[cfg(all(feature = "package-2015-03", not(feature = "no-default-version")))]
pub use package_2015_03::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-08-preview")]
pub mod package_2019_08_preview;
#[cfg(all(feature = "package-2019-08-preview", not(feature = "no-default-version")))]
pub use package_2019_08_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-09-preview")]
pub mod package_2019_09_preview;
#[cfg(all(feature = "package-2019-09-preview", not(feature = "no-default-version")))]
pub use package_2019_09_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-03-preview")]
pub mod package_2020_03_preview;
#[cfg(all(feature = "package-2020-03-preview", not(feature = "no-default-version")))]
pub use package_2020_03_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-12-01-preview")]
pub mod package_2021_12_01_preview;
#[cfg(all(feature = "package-2021-12-01-preview", not(feature = "no-default-version")))]
pub use package_2021_12_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-08")]
pub mod package_2020_08;
#[cfg(all(feature = "package-2020-08", not(feature = "no-default-version")))]
pub use package_2020_08::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-10-only")]
pub mod package_2020_10_only;
#[cfg(all(feature = "package-2020-10-only", not(feature = "no-default-version")))]
pub use package_2020_10_only::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-10")]
pub mod package_2020_10;
#[cfg(all(feature = "package-2020-10", not(feature = "no-default-version")))]
pub use package_2020_10::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "no-default-version")))]
pub use package_2021_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
