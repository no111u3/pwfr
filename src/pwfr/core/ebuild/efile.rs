//! Ebuild and Eclass file parser

#[derive(Parser)]
#[grammar = "pwfr/core/ebuild/efile.pest"]
pub struct EFile;
