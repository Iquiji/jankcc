use std::ops::Deref;

use cranelift::{
    codegen::ir::{Constant, ConstantPool},
    prelude::*,
};
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_object::ObjectModule;
use log::info;

use crate::mir::{MIR_Function, MIR_Instruction};

use super::CraneliftBackend;

impl CraneliftBackend {
    pub(crate) fn translate_function(
        &mut self,
        input: MIR_Function,
        constant_pool: &mut ConstantPool,
    ) {
        // Our toy language currently only supports I64 values, though Cranelift
        // supports other types.
        let int = types::I32;

        // for _p in &params {
        //     self.ctx.func.signature.params.push(AbiParam::new(int));
        // }

        // Our toy language currently only supports one return value, though
        // Cranelift is designed to support more.
        self.ctx.func.signature.returns.push(AbiParam::new(int));

        // Create the builder to build a function.
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        // Create the entry block, to start emitting code in.
        let entry_block = builder.create_block();

        // Since this is the entry block, add block parameters corresponding to
        // the function's parameters.
        builder.append_block_params_for_function_params(entry_block);

        // Tell the builder to emit code in this block.
        builder.switch_to_block(entry_block);

        // And, tell the builder that this block will have no further
        // predecessors. Since it's the entry block, it won't have any
        // predecessors.
        builder.seal_block(entry_block);

        for instr in &input.blocks[0].instr {
            CraneliftBackend::translate_instruction(
                instr.clone(),
                &mut builder,
                &mut self.module,
                constant_pool,
            );
        }

        // Tell the builder we're done with this function.
        builder.finalize();

        info!("{}", self.ctx.func);
    }
    pub(crate) fn translate_instruction(
        instr: MIR_Instruction,
        func_builder: &mut FunctionBuilder,
        module: &mut ObjectModule,
        constant_pool: &mut ConstantPool,
    ) {
        match instr {
            MIR_Instruction::Return(arg) => {
                let return_value = match arg {
                    crate::mir::MIR_Location::Global(_, _) => todo!(),
                    crate::mir::MIR_Location::Constant(val, val_type) => {
                        let int = types::I32;
                        func_builder.ins().iconst(int, val)
                    }
                    crate::mir::MIR_Location::ConstantLocation(_, _) => todo!(),
                    crate::mir::MIR_Location::Local(_, _) => todo!(),
                };
                func_builder.ins().return_(&[return_value]);
            }
            MIR_Instruction::Call(return_loc, symbol, arg_locations) => {
                // func_builder.ins().call(, args)
                let mut sig = module.make_signature();

                // Add a parameter for each argument.
                // for _arg in &args {
                sig.params.push(AbiParam::new(types::I64));
                // }

                // For simplicity for now, just make all calls return a single I64.
                sig.returns.push(AbiParam::new(types::I32));

                let callee = module
                    .declare_function(&symbol, Linkage::Export, &sig)
                    .map_err(|e| e.to_string())
                    .unwrap();

                let local_callee = module.declare_func_in_func(callee, &mut func_builder.func);

                let mut arg_values = Vec::new();
                for arg in arg_locations {
                    arg_values.push(match arg {
                        crate::mir::MIR_Location::Global(_, _) => todo!(),
                        crate::mir::MIR_Location::Constant(val, val_type) => {
                            let int = types::I32;
                            func_builder.ins().iconst(int, val)
                        }
                        crate::mir::MIR_Location::ConstantLocation(const_num, _val_type) => {
                            let sym = module
                                .declare_anonymous_data(false, false)
                                .expect("problem declaring data object");

                            let mut data_ctx = DataContext::new();
                            data_ctx.define(
                                constant_pool
                                    .get(Constant::from_u32(const_num as u32))
                                    .clone()
                                    .as_slice()
                                    .to_vec()
                                    .into_boxed_slice(),
                            );
                            module.define_data(sym, &data_ctx).unwrap();

                            let local_id = module.declare_data_in_func(sym, func_builder.func);

                            let pointer = module.target_config().pointer_type();
                            func_builder.ins().symbol_value(pointer, local_id)
                        }
                        crate::mir::MIR_Location::Local(_, _) => todo!(),
                    });
                }
                let call = func_builder.ins().call(local_callee, &arg_values);
                let _return_value = func_builder.inst_results(call)[0];
            }
        }
    }
}
