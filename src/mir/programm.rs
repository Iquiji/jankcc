use serde::{Deserialize, Serialize};

use crate::environment_builder::ext_type::{ExtType, PrettyType};

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
