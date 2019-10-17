//! Ebuild and Eclass builtins
//!
//! Builtins internals
//! Inspired by https://crates.io/crates/nsh (github: https://github.com/seiyanuta/nsh)

use std::collections::BTreeMap;

use crate::core::ebuild::executor::{Executor, ExitStatus};
use crate::core::ebuild::utils::FdFile;

pub struct InternalCommandContext<'a> {
    pub argv: &'a [String],
    pub isolate: &'a mut Executor,
    pub stdin: FdFile,
    pub stdout: FdFile,
    pub stderr: FdFile,
}

#[derive(Debug, Fail)]
pub enum InternalCommandError {
    #[fail(display = "command not found")]
    NotFound,
    #[fail(display = "failed to create redirections")]
    BadRedirection,
}

type InternalCommand = fn(&mut InternalCommandContext<'_>) -> ExitStatus;

lazy_static! {
    // TODO: Construct this map in compile time.
    pub static ref INTERNAL_COMMANDS: BTreeMap<&'static str, InternalCommand> = {
        let commands: BTreeMap<&'static str, InternalCommand> = BTreeMap::new();
        commands
    };
}

#[cfg(test)]
mod tests {}
