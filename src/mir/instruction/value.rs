use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MIRValue {
    pub(crate) opaque_ref: u32,
}
