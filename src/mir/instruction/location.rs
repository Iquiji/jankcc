use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MIRLocatorValue {
    LocalVar(LocalRef,PrettyType),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Copy)]
pub(crate) struct LocalRef {
    pub(crate) opaque_ref: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Copy)]
pub(crate) struct DataConstantRef {
    pub(crate) opaque_ref: u32,
}
