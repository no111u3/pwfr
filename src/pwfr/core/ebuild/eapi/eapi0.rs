//! Ebuild API 0
//!
//! Ebuild API 0 traits and facilities

use crate::core::ebuild::eapi::{EApi, EApiInfo, EApiVersion};

/// Ebuild API #0
///
/// Start implementation of API
pub trait EApi0: EApi {}

/// Ebulid API #0 implementation
pub struct EApi0impl;

impl EApi for EApi0impl {
    fn get_eapi_version(&self) -> EApiVersion {
        EApiVersion::EApi0
    }

    fn get_info(&self) -> Box<dyn EApiInfo> {
        unimplemented!();
    }
}

impl EApi0 for EApi0impl {}
