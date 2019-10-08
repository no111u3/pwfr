//! Ebuild API
//!
//! Ebuild API traits and facilities

pub mod eapi0;

/// Ebuild API versions
///
/// Contains implemented and supported Ebuild API versions
pub enum EApiVersion {
    EApi0,
}

/// Ebuild Package dependency
pub struct PackageDep {
    pub name: String,
    pub use_flags: Vec<String>,
}

/// Ebuild Information API
///
/// Provide main ebuild information
pub trait EApiInfo {
    fn get_name(&self) -> String;

    fn get_version(&self) -> String;

    fn get_use_flags(&self) -> Vec<String>;

    fn get_depends(&self, active_flags: Vec<String>) -> Vec<PackageDep>;
}

/// Main Ebuild API
pub trait EApi {
    fn get_eapi_version(&self) -> EApiVersion;

    fn get_info(&self) -> Box<dyn EApiInfo>;
}
