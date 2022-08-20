use log::warn;

use crate::parser::parse_nodes::TranslationUnit;
use crate::parser::parse_nodes::ExternalDeclaration::*;
use crate::parser::parse_nodes::declarations::Declaration;
use crate::parser::span::Spanned;


use super::EnvironmentController;

mod static_and_constant_expr;

impl EnvironmentController{
    pub(crate) fn walk_translation_unit(&mut self,translation_unit: TranslationUnit){
        for external_declaration in translation_unit{
            match &*external_declaration{
                FunctionDefinition(function_def) => todo!(),
                Declaration(declaration) => self.handle_external_declaration(declaration),
            }
        }
    }
    pub(crate) fn handle_external_declaration(&mut self,declaration: &Spanned<Declaration>){
        match &*declaration.inner{
            Declaration::Declaration { specifiers, init } => {
                if specifiers.storage.typedef_c{
                    warn!("Typedef: {:?} -> {:?}",init[0].0.base.identifier,specifiers.specifiers);
                }else{
                    warn!("Non Typedef");
                }
            },
            Declaration::StaticAssertDeclaration(static_assert) => self.handle_static_assert(static_assert),
        }
    }
}