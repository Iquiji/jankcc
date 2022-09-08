use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MIRProgramm {
    pub(crate) globals: Vec<GlobalEntity>,
    pub(crate) functions: Vec<MIRFunction>,
}

impl MIRProgramm {
    pub fn new() -> Self {
        Self {
            globals: vec![],
            functions: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct GlobalEntity {
    pub(crate) name: String,
    pub(crate) extern_linkage: bool,
}
