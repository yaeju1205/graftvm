use std::rc::Rc;

use graftvm_fragment::Fragment;
use graftvm_liternal::Liternal;

use crate::{opcode::Addr, vm::VM};

impl VM {
    pub(super) fn store_data(&mut self, index: usize, data: Liternal) {
        self.store_data_rc(index, Rc::new(data));
    }

    pub(super) fn store_data_rc(&mut self, index: usize, data: Rc<Liternal>) {
        self.constant_pool.insert(index, data);
    }

    pub(super) fn load_data(&mut self, dst: Addr, index: usize) -> Result<(), String> {
        if let Some(data) = self.constant_pool.get(&index) {
            self.load_data_rc(dst, data.clone())?;
        }

        Ok(())
    }

    pub(super) fn load_data_rc(&mut self, dst: Addr, data: Rc<Liternal>) -> Result<(), String> {
        *self.get_slot_mut(dst)? = Some(Fragment::from(data));

        Ok(())
    }
}
