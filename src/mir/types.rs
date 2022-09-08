use serde::{Deserialize, Serialize};

use crate::environment_builder::ext_type::PrettyType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MIRType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
}

impl MIRType {
    pub(crate) fn extract_from_pretty_type(p_type: &PrettyType) -> Self {
        use crate::environment_builder::ext_type::*;
        match &p_type.inner_type {
            ExtType::Void => todo!(),
            ExtType::Int {
                is_const,
                is_volatile,
                signed,
                size,
            } => {
                use MIRType::*;
                if *signed {
                    match size {
                        1 => I8,
                        2 => I16,
                        4 => I32,
                        8 => I64,
                        _ => unreachable!(),
                    }
                } else {
                    match size {
                        1 => U8,
                        2 => U16,
                        4 => U32,
                        8 => U64,
                        _ => unreachable!(),
                    }
                }
            }
            ExtType::Float {
                is_const,
                is_volatile,
                size,
            } => todo!(),
            ExtType::Array {
                is_const,
                is_volatile,
                arr_size,
                to,
            } => todo!(),
            ExtType::Pointer {
                is_const,
                is_volatile,
                to,
            } => MIRType::I64,
            ExtType::Function {
                overextendable,
                returns,
                parameters,
            } => todo!(),
            ExtType::Struct {
                is_const,
                is_volatile,
                tag,
                alignment,
                members,
            } => todo!(),
            ExtType::Union {
                is_const,
                is_volatile,
                tag,
                members,
            } => todo!(),
        }
    }
}
