use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MIRBlock {
    pub(crate) instr: Vec<MIRInstruction>,
    pub(crate) branches: Option<(MIRValue, Vec<MIRBranch>)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MIRBranch {
    pub(crate) is_default: bool,
    pub(crate) value_needed: u64,
    /// index of block in function
    pub(crate) to_block: usize,
}

impl MIRBlock {
    pub(crate) fn new_simple() -> Self {
        MIRBlock {
            instr: vec![],
            branches: None,
        }
    }
    pub(crate) fn new_wrapped() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(MIRBlock::new_simple()))
    }
}

impl MIRBlock {
    pub(crate) fn ins_instr(rc_self: Rc<RefCell<Self>>, instr: MIRInstruction) {
        rc_self.borrow_mut().instr.push(instr);
    }
}
