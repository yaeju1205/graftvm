use graftvm_bytecode::Addr;
use graftvm_liternal::Liternal;
use graftvm_window::WindowSlot;

use crate::vm::VM;

impl VM {
    pub(super) fn store_data(&mut self, index: usize, data: Liternal) {
        self.constant_pool.insert(index, data);
    }

    pub(super) fn load_data(&mut self, dst: Addr, index: usize) -> Result<(), String> {
        if let Some(data) = self.constant_pool.get(&index) {
            *self.slot_mut(dst.slot) = Some(WindowSlot::from(data.clone()));
        }
        Ok(())
    }
}
