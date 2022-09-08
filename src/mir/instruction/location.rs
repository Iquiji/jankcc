use super::*;

// #[allow(non_camel_case_types)]
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub(crate) enum MIR_Location {
//     Global(usize, MIR_Type),
//     /// Needs to be Pointer
//     ConstantLocation(usize, MIR_Type),
//     Local(String, MIR_Type),
// }
// impl MIR_Location {
//     pub(crate) fn get_mir_type(&self) -> MIR_Type {
//         match self {
//             MIR_Location::Global(_, mir_type) => mir_type.clone(),
//             MIR_Location::ConstantLocation(_, mir_type) => mir_type.clone(),
//             MIR_Location::Local(_, mir_type) => mir_type.clone(),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LocalRef {
    pub(crate) opaque_ref: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DataConstantRef {
    pub(crate) opaque_ref: u32,
}
