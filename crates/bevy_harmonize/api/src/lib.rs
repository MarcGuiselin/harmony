#![allow(dead_code)] // TODO: remove
#![feature(const_trait_impl)]
#![feature(const_type_name)]

#[cfg(all(feature = "generate_manifest", feature = "wasm_runtime"))]
compile_error!(
    "Features \"generate_manifest\" and \"wasm_runtime\" cannot be enabled at the same time"
);

pub mod ecs;
pub mod init;
pub(crate) mod utils;

pub mod prelude {
    pub use crate::ecs::{system::Commands, Component};
    pub use crate::init::{Feature, FeatureBuilder, Harmony};
    pub use common::{HasStableId, Start, Update};
}
