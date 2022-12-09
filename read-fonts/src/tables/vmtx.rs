//! The [vmtx (Vertical Metrics)](https://docs.microsoft.com/en-us/typography/opentype/spec/vmtx) table

use types::Tag;

pub use super::hmtx::LongMetric;

/// 'vmtx'
pub const TAG: Tag = Tag::new(b"vmtx");

include!("../../generated/generated_vmtx.rs");
