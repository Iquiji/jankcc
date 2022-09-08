use std::{collections::HashMap};

use cranelift::{
    codegen::ir::{ConstantPool},
    prelude::{isa::CallConv, *},
};

use cranelift_object::ObjectModule;
use log::info;

use crate::mir::{MIRFunction, MIRInstruction, MIRType};

use super::CraneliftBackend;

impl MIRType {
    pub(crate) fn into_cranelift_type(&self) -> Type {
        match self {
            MIRType::U8 => todo!(),
            MIRType::I8 => todo!(),
            MIRType::U16 => todo!(),
            MIRType::I16 => todo!(),
            MIRType::U32 => todo!(),
            MIRType::I32 => types::I32,
            MIRType::U64 => todo!(),
            MIRType::I64 => types::I64,
        }
    }
}

impl CraneliftBackend {
    pub(crate) fn translate_function(
        &mut self,
        input: MIRFunction,
        constant_pool: &mut ConstantPool,
    ) {
        self.ctx.func.clear();
        self.ctx.func.signature.call_conv = CallConv::SystemV;

        // add signature
        for p in &input.signature.args {
            self.ctx
                .func
                .signature
                .params
                .push(AbiParam::new(p.into_cranelift_type()));
        }

        // Our toy language currently only supports one return value, though
        // Cranelift is designed to support more.
        self.ctx.func.signature.returns.push(AbiParam::new(
            input.signature.return_type.into_cranelift_type(),
        ));

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

        // var map for later
        let mut var_map: HashMap<String, Variable> = HashMap::new();
        let mut var_idx_counter = 0;
        // register function parameters as variables
        for (param_idx, (param, param_type)) in input
            .parameter_names
            .iter()
            .zip(input.signature.args.iter())
            .enumerate()
        {
            let val = builder.block_params(entry_block)[param_idx];
            let var = Variable::new(var_idx_counter);
            if let std::collections::hash_map::Entry::Vacant(e) = var_map.entry(param.to_string()) {
                e.insert(var);
                builder.declare_var(var, param_type.into_cranelift_type());
                builder.def_var(var, val);
                var_idx_counter += 1;
            }
        }

        // register all function variables
        // for var_name in input.vars {
        //     let var = Variable::new(var_idx_counter);
        //     if let std::collections::hash_map::Entry::Vacant(e) = var_map.entry(var_name.0) {
        //         e.insert(var);
        //         builder.declare_var(var, var_name.1.into_cranelift_type());
        //         var_idx_counter += 1;
        //     }
        // }

        let translator = CraneliftFunctionTranslator {
            func_builder: &mut builder,
            module: &mut self.module,
            constant_pool,
            var_map: &mut var_map,
        };

        // for instr in &input.blocks[0].instr {
        //     translator.translate_instruction(instr.clone());
        // }

        // Tell the builder we're done with this function.
        translator.func_builder.finalize();

        info!("{}", self.ctx.func);
    }
}

pub(crate) struct CraneliftFunctionTranslator<'a> {
    pub(crate) func_builder: &'a mut FunctionBuilder<'a>,
    pub(crate) module: &'a mut ObjectModule,
    pub(crate) constant_pool: &'a mut ConstantPool,
    pub(crate) var_map: &'a mut HashMap<String, Variable>,
}
impl CraneliftFunctionTranslator<'_> {
    pub(crate) fn translate_instruction(&mut self, _instr: MIRInstruction) {
        todo!()
        // match instr {
        //     MIRInstruction::Return(arg) => {
        //         let return_value = self.into_cranelift_value(arg);
        //         self.func_builder.ins().return_(&[return_value]);
        //     }
        //     MIRInstruction::Call(return_loc, symbol, arg_locations, mirsignature) => {
        //         // func_builder.ins().call(, args)
        //         let mut sig = self.module.make_signature();

        //         // Add a parameter for each argument.
        //         if !mirsignature.overloadable {
        //             for p in mirsignature.args {
        //                 sig.params.push(AbiParam::new(p.into_cranelift_type()));
        //             }
        //         } else {
        //             for parg in &arg_locations {
        //                 sig.params
        //                     .push(AbiParam::new(parg.get_mirtype().into_cranelift_type()));
        //             }
        //         }

        //         // return type of called function
        //         sig.returns.push(AbiParam::new(
        //             mirsignature.return_type.into_cranelift_type(),
        //         ));

        //         let callee = self
        //             .module
        //             .declare_function(&symbol, Linkage::Export, &sig)
        //             .map_err(|e| e.to_string())
        //             .unwrap();

        //         let local_callee = self
        //             .module
        //             .declare_func_in_func(callee, self.func_builder.func);

        //         let mut arg_values = Vec::new();
        //         for arg in arg_locations {
        //             arg_values.push(self.into_cranelift_value(arg));
        //         }
        //         let call = self.func_builder.ins().call(local_callee, &arg_values);
        //         let return_value = self.func_builder.inst_results(call)[0];
        //         if let MIRLocation::Local(name, mirtype) = return_loc {
        //             let variable = self.var_map.get(&name).unwrap();
        //             self.func_builder.def_var(*variable, return_value);
        //         } else {
        //             unimplemented!()
        //         }
        //     }
        //     MIRInstruction::Add(add_res_loc, left_loc, right_loc) => {
        //         let left_value = self.into_cranelift_value(right_loc);
        //         let right_value = self.into_cranelift_value(left_loc);
        //         let add_res = self.func_builder.ins().iadd(left_value, right_value);
        //         if let MIRLocation::Local(name, mirtype) = add_res_loc {
        //             let variable = self.var_map.get(&name).unwrap();
        //             self.func_builder.def_var(*variable, add_res);
        //         } else {
        //             unimplemented!()
        //         }
        //     }
        //     MIRInstruction::Assign(to_assign_loc, from_loc) => {
        //         if let MIRLocation::Local(name, mirtype) = to_assign_loc {
        //             let assign_value = self.into_cranelift_value(from_loc);
        //             let variable = self.var_map.get(&name).unwrap();
        //             self.func_builder.def_var(*variable, assign_value);
        //         } else {
        //             unimplemented!()
        //         }
        //     }
        // }
    }
}
