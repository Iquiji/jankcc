use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub(crate) struct MIRValue {
    pub(crate) opaque_ref: u32,
}
