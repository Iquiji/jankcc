use cranelift::prelude::*;

use crate::mir::MIR_Location;

impl MIR_Location{
    pub(crate) fn into_cranelift_value(&self,func_builder: &mut FunctionBuilder) -> Value{
        // TODO: this
        match self {
            crate::mir::MIR_Location::Global(_, _) => todo!(),
            crate::mir::MIR_Location::Constant(val, val_type) => {
                func_builder.ins().iconst(val_type.into_cranelift_type(), *val)
            }
            crate::mir::MIR_Location::ConstantLocation(_, _) => todo!(),
            crate::mir::MIR_Location::Local(_, _) => todo!(),
        }
    }
}