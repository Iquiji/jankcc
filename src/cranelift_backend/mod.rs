mod helpers;
mod translate_function;

use std::error::Error;

use log::error;

use cranelift::{
    codegen::ir::{ConstantData, ConstantPool},
    prelude::*,
};
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

use crate::{mir::MIRProgramm, parser::parse_nodes::Constant};

/// The basic Object class.
pub struct CraneliftBackend {
    /// The function builder context, which is reused across multiple
    /// FunctionBuilder instances.
    builder_context: FunctionBuilderContext,

    /// The main Cranelift context, which holds the state for codegen. Cranelift
    /// separates this from `Module` to allow for parallel compilation, with a
    /// context per thread, though this isn't in the simple demo here.
    ctx: codegen::Context,

    /// The data context, which is to data objects what `ctx` is to functions.
    data_ctx: DataContext,

    /// The module, with the Object backend, which manages the Object'd
    /// functions.
    module: ObjectModule,
}

impl Default for CraneliftBackend {
    fn default() -> Self {
        let mut settings_builder = settings::builder();
        settings_builder.set("opt_level", "speed").unwrap();
        let builder = ObjectBuilder::new(
            cranelift::codegen::isa::lookup_by_name("x86_64-linux")
                .unwrap()
                .finish(settings::Flags::new(settings_builder))
                .unwrap(),
            "x64",
            cranelift_module::default_libcall_names(),
        );
        let module = ObjectModule::new(builder.unwrap());
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        }
    }
}

impl CraneliftBackend {
    /// Compile a string in the toy language into machine code.
    pub(crate) fn compile(&mut self, input: MIRProgramm) {
        for global in input.globals {
            if global.extern_linkage {}
        }
        // println!("before func: {}", self.ctx.func);
        for function in &input.functions {
            self.translate_function(function.clone());

            // Next, declare the function to Object. Functions must be declared
            // before they can be called, or defined.
            let id = self
                .module
                .declare_function(&function.name, Linkage::Export, &self.ctx.func.signature)
                .map_err(|e| e.to_string())
                .unwrap();

            // Define the function to Object. This finishes compilation, although
            // there may be outstanding relocations to perform. Currently, Object
            // cannot finish relocations until all functions to be called are
            // defined. For this toy demo for now, we'll just finalize the
            // function below.
            let errors = self.module.define_function(id, &mut self.ctx);
            if let Err(error) = errors {
                error!("{:?}", error.source());
            }
            // Now that compilation is finished, we can clear out the context state.
            self.module.clear_context(&mut self.ctx);
        }
    }
    pub(crate) fn finish(self) -> Vec<u8> {
        // Finalize the functions which we just defined, which resolves any
        // outstanding relocations (patching in addresses, now that they're
        // available).
        let object = self.module.finish();

        // We can now retrieve a pointer to the machine code.
        let object_data = object.emit();

        object_data.unwrap()
    }
}
