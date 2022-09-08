use std::collections::{BTreeMap, HashMap};

use cranelift::{
    codegen::ir::{Constant, ConstantData, ConstantPool},
    prelude::{isa::CallConv, *},
};

use cranelift_module::{DataContext, Linkage, Module};
use cranelift_object::ObjectModule;
use log::info;

use crate::mir::{MIRFunction, MIRInstruction, MIRType, MIRValue};

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
    pub(crate) fn translate_function(&mut self, input: MIRFunction) {
        info!("translating function: {}",input.name);

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
        for var_local_ref_pair in &input.var_name_id_map {
            let mir_type_of_var = input.var_type_map.get(var_local_ref_pair.0).unwrap();
            let var = Variable::new(var_idx_counter);
            if let std::collections::hash_map::Entry::Vacant(e) =
                var_map.entry(var_local_ref_pair.1.to_string())
            {
                e.insert(var);
                builder.declare_var(var, mir_type_of_var.into_cranelift_type());
                var_idx_counter += 1;
            }
        }

        let mut translator = CraneliftFunctionTranslator {
            mir_function: &input,
            func_builder: &mut builder,
            module: &mut self.module,
            var_map: &mut var_map,
            value_trans_map: BTreeMap::new(),
        };

        for instr in &translator.mir_function.blocks[0].borrow().instr {
            translator.translate_instruction(instr.clone());
        }

        // Tell the builder we're done with this function.
        translator.func_builder.finalize();

        info!("{}", self.ctx.func);
    }
}

pub(crate) struct CraneliftFunctionTranslator<'a> {
    pub(crate) mir_function: &'a MIRFunction,
    pub(crate) func_builder: &'a mut FunctionBuilder<'a>,
    pub(crate) module: &'a mut ObjectModule,
    pub(crate) var_map: &'a mut HashMap<String, Variable>,
    pub(crate) value_trans_map: BTreeMap<MIRValue, Value>,
}
impl CraneliftFunctionTranslator<'_> {
    pub(crate) fn translate_instruction(&mut self, instr: MIRInstruction) {
        match instr {
            MIRInstruction::GetConstDataPtr(output_mir_value, const_ref) => {
                // we first declare the data before we can use it
                let sym = self
                    .module
                    .declare_anonymous_data(false, false)
                    .expect("problem declaring data object");

                // we then get the data from the reference in the map and define the DataId
                let mut data_ctx = DataContext::new();
                data_ctx.define(
                    self.mir_function
                        .data_const_id_map
                        .get_by_left(&const_ref)
                        .unwrap_or_else(|| panic!("integrity error with constant map"))
                        .value
                        .clone()
                        .into_boxed_slice(),
                );
                self.module.define_data(sym, &data_ctx).unwrap();

                // we then use it and get a pointer to it
                let local_id = self
                    .module
                    .declare_data_in_func(sym, self.func_builder.func);
                let pointer = self.module.target_config().pointer_type();

                // we then convert the pointer to a value and enter it into the value translation map
                let output_value = self.func_builder.ins().symbol_value(pointer, local_id);
                self.insert_value_trans_pair(output_mir_value, output_value);
            }
            MIRInstruction::ConstNum(mir_value, num, num_type) => {
                // integer Constant
                let cranelift_value = self
                    .func_builder
                    .ins()
                    .iconst(num_type.into_cranelift_type(), num);
                self.insert_value_trans_pair(mir_value, cranelift_value);
            }
            MIRInstruction::ReadLocal(mir_output_value, mir_local_ref) => {
                let variable_name = self
                    .mir_function
                    .var_name_id_map
                    .get_by_left(&mir_local_ref)
                    .unwrap_or_else(|| {
                        panic!("variable not defined => impossible: {:?}", mir_local_ref)
                    });
                let variable = self.var_map.get(variable_name).unwrap_or_else(|| {
                    panic!("variable not defined => impossible: {}", variable_name)
                });

                // use var
                let cranelift_output_value = self.func_builder.use_var(*variable);

                // gen link
                self.insert_value_trans_pair(mir_output_value, cranelift_output_value);
            }
            MIRInstruction::AssignLocal(mir_local_ref, mir_value_to_assign_to) => {
                let variable_name = self
                    .mir_function
                    .var_name_id_map
                    .get_by_left(&mir_local_ref)
                    .unwrap_or_else(|| {
                        panic!("variable not defined => impossible: {:?}", mir_local_ref)
                    });
                let variable = self.var_map.get(variable_name).unwrap_or_else(|| {
                    panic!("variable not defined => impossible: {}", variable_name)
                });

                let cranelift_assign_value = self.mir_value_to_cranelift_value(mir_value_to_assign_to);

                self.func_builder.def_var(*variable, cranelift_assign_value);
            },
            MIRInstruction::Add(result_mir_value, left_mir_value, right_mir_value) => {
                let left_value = self.mir_value_to_cranelift_value(left_mir_value);
                let right_value = self.mir_value_to_cranelift_value(right_mir_value);
                let add_value = self.func_builder.ins().iadd(left_value, right_value);
                self.insert_value_trans_pair(result_mir_value, add_value);
            },
            MIRInstruction::Call(
                mir_return_value,
                func_name,
                arg_values,
                signature_of_function,
            ) => {
                // direct call to function with known direct name

                // make a new signature for the function call
                let mut sig = self.module.make_signature();

                // finish the signature with type
                if !signature_of_function.overloadable {
                    for p in signature_of_function.args {
                        sig.params.push(AbiParam::new(p.into_cranelift_type()));
                    }
                } else {
                    for parg in &arg_values {
                        // for the value type of each argument get the type
                        sig.params.push(AbiParam::new(
                            self.mir_function
                                .value_type_map
                                .get(parg)
                                .unwrap_or_else(|| panic!("no MIRType for MIRValue: {:?}",parg))
                                .into_cranelift_type(),
                        ));
                    }
                }

                // return type of called function
                sig.returns.push(AbiParam::new(
                    signature_of_function.return_type.into_cranelift_type(),
                ));

                let callee = self
                    .module
                    .declare_function(&func_name, Linkage::Export, &sig)
                    .map_err(|e| e.to_string())
                    .unwrap();
                let local_callee = self
                    .module
                    .declare_func_in_func(callee, self.func_builder.func);

                // generate the Cranelift Values for the call
                let mut call_args = vec![];
                for mir_arg_value in arg_values {
                    let cranelift_value = self.mir_value_to_cranelift_value(mir_arg_value);
                    call_args.push(cranelift_value);
                }

                // call the function
                let call = self.func_builder.ins().call(local_callee, &call_args);
                let cranelift_return_value = self.func_builder.inst_results(call)[0];
                self.insert_value_trans_pair(mir_return_value, cranelift_return_value);
            }
            MIRInstruction::Return(mir_value) => {
                let cranelift_value = self.mir_value_to_cranelift_value(mir_value);
                self.func_builder.ins().return_(&[cranelift_value]);
            }
        }
    }
}

impl CraneliftFunctionTranslator<'_> {
    pub(crate) fn mir_value_to_cranelift_value(&self, mir_value: MIRValue) -> Value {
        *self.value_trans_map.get(&mir_value).unwrap()
    }
    pub(crate) fn insert_value_trans_pair(&mut self, mir_value: MIRValue, cranelift_value: Value) {
        self.value_trans_map.insert(mir_value, cranelift_value);
    }
}
