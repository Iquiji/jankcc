use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MIRFunction {
    pub(crate) name: String,
    pub(crate) signature: MIRSignature,
    pub(crate) parameter_names: Vec<String>,
    pub(crate) var_name_id_map: BiBTreeMap<LocalRef, String>,
    pub(crate) var_type_map: BTreeMap<LocalRef, MIRType>,
    pub(crate) var_type_map_pretty: BTreeMap<LocalRef, PrettyType>,
    pub(crate) data_const_id_map: BTreeMap<DataConstantRef, MIRConstant>,
    pub(crate) blocks: Vec<Rc<RefCell<MIRBlock>>>,
    pub(crate) current_block: Rc<RefCell<MIRBlock>>,
    pub(crate) ctx_gen: MIRFunctionContextGenerator,
    pub(crate) value_type_map: BTreeMap<MIRValue, MIRType>,
    pub(crate) value_type_map_pretty: BTreeMap<MIRValue, PrettyType>,
}
impl MIRFunction {
    pub(crate) fn new() -> MIRFunction {
        let origin_block = MIRBlock::new_wrapped();
        MIRFunction {
            name: String::new(),
            signature: MIRSignature {
                return_type: MIRType::I64,
                args: vec![],
                overloadable: false,
            },
            parameter_names: vec![],
            var_name_id_map: BiBTreeMap::new(),
            var_type_map: BTreeMap::new(),
            var_type_map_pretty: BTreeMap::new(),
            data_const_id_map: BTreeMap::new(),
            blocks: vec![origin_block.clone()],
            current_block: origin_block,
            ctx_gen: MIRFunctionContextGenerator::new(),
            value_type_map: BTreeMap::new(),
            value_type_map_pretty: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MIRFunctionContextGenerator {
    pub(crate) intermediate_value_counter: u32,
    pub(crate) data_const_ref_counter: u32,
    pub(crate) var_ref_counter: u32,
}

impl MIRFunctionContextGenerator {
    pub(crate) fn new() -> Self {
        MIRFunctionContextGenerator {
            intermediate_value_counter: 0,
            data_const_ref_counter: 0,
            var_ref_counter: 0,
        }
    }
    fn make_intermediate_value(&mut self) -> MIRValue {
        let temp = MIRValue {
            opaque_ref: self.intermediate_value_counter,
        };
        self.intermediate_value_counter += 1;
        temp
    }

    pub(crate) fn make_data_const_ref(&mut self) -> DataConstantRef {
        let temp = DataConstantRef {
            opaque_ref: self.data_const_ref_counter,
        };
        self.data_const_ref_counter += 1;
        temp
    }
    pub(crate) fn make_var_ref(&mut self) -> LocalRef {
        let temp = LocalRef {
            opaque_ref: self.var_ref_counter,
        };
        self.var_ref_counter += 1;
        temp
    }
}

impl MIRFunction {
    pub(crate) fn make_intermediate_value_typed(&mut self, p_type: PrettyType) -> MIRValue {
        let value = self.ctx_gen.make_intermediate_value();
        self.value_type_map
            .insert(value, MIRType::extract_from_pretty_type(&p_type));
        self.value_type_map_pretty.insert(value, p_type);
        value
    }
    pub(crate) fn insert_constant(&mut self, constant: MIRConstant) -> DataConstantRef {
        let c_ref = self.ctx_gen.make_data_const_ref();
        self.data_const_id_map.insert(c_ref, constant);
        //.expect("internal data const ref error");
        c_ref
    }
    pub(crate) fn insert_variable(&mut self, var: String, var_type: PrettyType) -> LocalRef {
        let var_ref = self.ctx_gen.make_var_ref();
        self.var_name_id_map
            .insert_no_overwrite(var_ref, var)
            .expect("internal var ref error");
        self.var_type_map
            .insert(var_ref, MIRType::extract_from_pretty_type(&var_type));
        self.var_type_map_pretty.insert(var_ref, var_type);
        var_ref
    }
}
