use std::collections::HashMap;

use cranelift::{codegen::ir::Constant, prelude::*};
use cranelift_module::{DataContext, Module};

use crate::mir::MIR_Location;

use super::translate_function::CraneliftFunctionTranslator;

impl CraneliftFunctionTranslator<'_> {
    pub(crate) fn into_cranelift_value(&mut self, mir_location: MIR_Location) -> Value {
        // TODO: this
        match mir_location {
            crate::mir::MIR_Location::Global(_, _) => todo!(),
            crate::mir::MIR_Location::Constant(val, val_type) => self
                .func_builder
                .ins()
                .iconst(val_type.into_cranelift_type(), val),
            crate::mir::MIR_Location::ConstantLocation(const_num, _val_type) => {
                let sym = self
                    .module
                    .declare_anonymous_data(false, false)
                    .expect("problem declaring data object");

                let mut data_ctx = DataContext::new();
                data_ctx.define(
                    self.constant_pool
                        .get(Constant::from_u32(const_num as u32))
                        .clone()
                        .as_slice()
                        .to_vec()
                        .into_boxed_slice(),
                );
                self.module.define_data(sym, &data_ctx).unwrap();

                let local_id = self
                    .module
                    .declare_data_in_func(sym, self.func_builder.func);

                let pointer = self.module.target_config().pointer_type();
                self.func_builder.ins().symbol_value(pointer, local_id)
            }
            crate::mir::MIR_Location::Local(name, mir_type) => {
                let variable = self
                    .var_map
                    .get(&name)
                    .unwrap_or_else(|| panic!("variable not defined => impossible: {}", name));
                self.func_builder.use_var(*variable)
            }
        }
    }
}
