// #![deny(
//     warnings,
//     missing_copy_implementations,
//     trivial_casts,
//     trivial_numeric_casts,
//     unsafe_code,
//     unstable_features,
//     unused_import_braces,
//     unused_qualifications,
//     missing_docs
// )]

//! Nine Patch sprite system for bevy

mod ninepatch;
pub use ninepatch::{GrowthMode, NinePatch, NinePatchBuilder, NinePatchContent, Patch, PatchSize};

mod plugin;
pub use plugin::*;
