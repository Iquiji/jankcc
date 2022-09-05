use std::ops::Deref;

use cranelift::{
    codegen::ir::{Constant, ConstantPool, Function},
    prelude::{*, isa::CallConv},
};
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_object::ObjectModule;
use log::info;

use crate::mir::{MIR_Function, MIR_Instruction, MIR_Type};

use super::CraneliftBackend;

impl MIR_Type{
    pub(crate) fn into_cranelift_type(&self) -> Type{
        match self{
            MIR_Type::u8 => todo!(),
            MIR_Type::i8 => todo!(),
            MIR_Type::u16 => todo!(),
            MIR_Type::i16 => todo!(),
            MIR_Type::u32 => todo!(),
            MIR_Type::i32 => types::I32,
            MIR_Type::u64 => todo!(),
            MIR_Type::i64 => types::I64,
        }
    }
}


impl CraneliftBackend {
    pub(crate) fn translate_function(
        &mut self,
        input: MIR_Function,
        constant_pool: &mut ConstantPool,
    ) {
        self.ctx.func.clear();
        self.ctx.func.signature.call_conv = CallConv::SystemV;
        // Our toy language currently only supports I64 values, though Cranelift
        // supports other types.
        for p in input.signature.args{
            self.ctx.func.signature.params.push(AbiParam::new(p.into_cranelift_type()));
        }

        // Our toy language currently only supports one return value, though
        // Cranelift is designed to support more.
        self.ctx.func.signature.returns.push(AbiParam::new(input.signature.return_type.into_cranelift_type()));

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
                let return_value = arg.into_cranelift_value(func_builder);
                func_builder.ins().return_(&[return_value]);
            }
            MIR_Instruction::Call(return_loc, symbol, arg_locations,mir_signature) => {
                // func_builder.ins().call(, args)
                let mut sig = module.make_signature();

                // Add a parameter for each argument.
                for p in mir_signature.args{
                    sig.params.push(AbiParam::new(p.into_cranelift_type()));
                }
        
                // return type of called function
                sig.returns.push(AbiParam::new(mir_signature.return_type.into_cranelift_type()));

                let callee = module
                    .declare_function(&symbol, Linkage::Export, &sig)
                    .map_err(|e| e.to_string())
                    .unwrap();

                let local_callee = module.declare_func_in_func(callee, func_builder.func);

                let mut arg_values = Vec::new();
                for arg in arg_locations {
                    arg_values.push(match arg {
                        crate::mir::MIR_Location::Global(_, _) => todo!(),
                        crate::mir::MIR_Location::Constant(val, val_type) => {
                            let int = val_type.into_cranelift_type();
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
            MIR_Instruction::Add(return_loc, left_loc, right_loc) => {
                let left_value = left_loc.into_cranelift_value(func_builder);
                let right_value = right_loc.into_cranelift_value(func_builder);
                func_builder.ins().iadd(left_value, right_value);
            },
        }
    }
}
