/*
An identifier can denote:
    object;
    a function;
    a tag or a member of a structure, union, or enumeration;
    a typedef name;
    a label name;
*/

/*
different entities designated by the same identifier either have different scopes, or are in different name spaces.

There are four kinds of scopes:
    - function
    - file
    - block
    - function prototype. (A function prototype is a declaration of a function that declares the types of its parameters.)
*/

/*
label name is the only kind of identifier that has function scope.
*/

/*
Structuring of Object in Scope:
- ident
- type
- Assignment enumeration
- completeness of type flag
- qualifiers
- Value if existant as Constant

*/

/*
Three Different Name Spaces for:
— label names (disambiguated by the syntax of the label declaration and use);
— the tags of structures, unions, and enumerations (disambiguated by following any of the keywords struct, union, or enum);
— the members of structures or unions; each structure or union has a separate name space for its members (disambiguated by the type of the expression used to access the member via the . or -> operator);
— all other identifiers, called ordinary identifiers (declared in ordinary declarators or as enumeration constants).
*/

use std::{cell::RefCell, collections::HashMap};

use log::debug;
use serde::{Deserialize, Serialize};

use crate::parser::{
    parse_nodes::declarations::{CAlignmentSpecifier, CFunctionSpecifier, DerivedDeclarator},
    types::{CTypeQualifiers, CTypeSpecifier},
};

use super::ext_type::PrettyType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct BlockContainer {
    pub(crate) scope: ScopeContainer,
    pub(crate) active_inner: Option<Box<BlockContainer>>,
    pub(crate) past_inner: Vec<BlockContainer>,
}
impl BlockContainer {
    pub(crate) fn new() -> Self {
        Self {
            scope: ScopeContainer::new(),
            active_inner: None,
            past_inner: vec![],
        }
    }
    pub(crate) fn get_current_scope(&mut self) -> &mut ScopeContainer {
        if let Some(active) = &mut self.active_inner {
            active.get_current_scope()
        } else {
            &mut self.scope
        }
    }
    pub(crate) fn enter_new_level(&mut self) {
        debug!("entered new scope level");
        if let Some(active) = &mut self.active_inner {
            active.enter_new_level();
        } else {
            self.active_inner = Some(Box::new(BlockContainer::new()));
        }
    }
    pub(crate) fn exit_new_level(&mut self) {
        debug!("exited new scope level");
        if let Some(active) = &mut self.active_inner {
            if active.active_inner.is_some() {
                active.exit_new_level();
            } else {
                let temp = *active.clone();
                self.active_inner = None;
                self.past_inner.push(temp);
            }
        }
    }
}

impl BlockContainer {
    pub(crate) fn get_top_typedefed(&mut self, ident: &str) -> Option<&RefCell<TypedefInstance>> {
        if let Some(active) = &mut self.active_inner {
            let res = active.get_top_typedefed(ident);
            if res.is_some() {
                return res;
            }
        }
        self.scope.typedefs.get(ident)
    }
    pub(crate) fn get_top_tag(&mut self, ident: &str) -> Option<&RefCell<TagInstance>> {
        if let Some(active) = &mut self.active_inner {
            let res = active.get_top_tag(ident);
            if res.is_some() {
                return res;
            }
        }
        self.scope.tags.get(ident)
    }
    pub(crate) fn get_top_variable(&mut self, ident: &str) -> Option<&RefCell<VariableInstance>> {
        let result = {
            if let Some(active) = &mut self.active_inner {
                let res = active.get_top_variable(ident);
                if res.is_some() {
                    return res;
                }
            }
            self.scope.variables.get(ident)
        };
        if let Some(result) = result {
            result.borrow_mut().usage_counter += 1;
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ScopeContainer {
    pub(crate) variables: HashMap<String, RefCell<VariableInstance>>,
    pub(crate) typedefs: HashMap<String, RefCell<TypedefInstance>>,
    pub(crate) tags: HashMap<String, RefCell<TagInstance>>,
    pub(crate) members: HashMap<String, MemberInstance>,
}

impl ScopeContainer {
    pub(crate) fn new() -> Self {
        Self {
            variables: HashMap::new(),
            typedefs: HashMap::new(),
            tags: HashMap::new(),
            members: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct VariableInstance {
    pub(crate) is_extern: bool,
    pub(crate) usage_counter: usize,
    pub(crate) associated_type: PrettyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TypedefInstance {
    pub(crate) def_type: PrettyType,
}

/// Must Refer to enum or union or struct :)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TagInstance {
    pub(crate) tag_type: PrettyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MemberInstance {}
