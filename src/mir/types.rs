use serde::{Deserialize, Serialize};

use crate::environment_builder::ext_type::{ExtType, PrettyType};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MIRType {
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
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
                        1 => i8,
                        2 => i16,
                        4 => i32,
                        8 => i64,
                        _ => unreachable!(),
                    }
                } else {
                    match size {
                        1 => u8,
                        2 => u16,
                        4 => u32,
                        8 => u64,
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
            } => MIRType::i64,
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
