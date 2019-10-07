//! Ebuild API
//!
//! Ebuild API traits and facilities

// Ebuild API versions
//
// Contains implemented and supported Ebuild API versions
pub enum EApiVersion {
}

// Main Ebuild API
pub trait EApi {
    fn get_eapi_version(&self) -> EApiVersion;
}
