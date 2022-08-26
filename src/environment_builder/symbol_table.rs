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

use std::collections::HashMap;

use crate::parser::{types::{CTypeQualifiers, CTypeSpecifier}, parse_nodes::declarations::{CFunctionSpecifier, CAlignmentSpecifier, DerivedDeclarator}};

pub(crate) struct BlockContainer {
    pub(crate) scope: ScopeContainer,
    pub(crate) active_inner: Option<Box<BlockContainer>>,
    pub(crate) past_inner: Vec<BlockContainer>,
}

pub(crate) struct ScopeContainer {
    pub(crate) variables: HashMap<String, VariableInstance>,
    pub(crate) typedefs: HashMap<String, TypedefInstance>,
    pub(crate) tags: HashMap<String, TagInstance>,
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

pub(crate) struct VariableInstance {}

pub(crate) struct TypedefInstance {
    pub(crate) qualifiers: CTypeQualifiers,
    pub(crate) specifier: CTypeSpecifier,
    pub(crate) func_spec: Option<CFunctionSpecifier>,
    pub(crate) alignment: Option<CAlignmentSpecifier>,
    pub(crate) derive: DerivedDeclarator,
}

pub(crate) struct TagInstance {}

pub(crate) struct MemberInstance {}
