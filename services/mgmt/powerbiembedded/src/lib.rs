#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2016-01")]
pub mod package_2016_01;
#[cfg(all(feature = "package-2016-01", not(feature = "no-default-version")))]
pub use package_2016_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
