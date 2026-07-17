use graftvm_data::kind::Kind;
use graftvm_error::RuntimeError;

use crate::{opcode::Addr, vm::VM};

impl VM {
    pub(super) fn add(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), RuntimeError> {
        if let Some(fragment) = self.get_slot(lhs)? {
            if !(fragment.is_int() || fragment.is_uint() || fragment.is_float()) {
                return Err(RuntimeError::new(format!(
                    "expected interger or float, found {:?}",
                    fragment.kind
                )));
            }
        } else {
            return Err(RuntimeError::new(
                "expected integer or float, found None".into(),
            ));
        }

        *self.get_slot_mut(dst)? = self.get_slot(src)
    }
}
