use std::rc::Rc;

use graftvm_bytecode::Addr;
use graftvm_window::{Window, WindowSlot};

use crate::vm::VM;

impl VM {
    fn check_slot(&self, src: &Addr) -> Result<(), String> {
        if self.window.state.id != src.window {
            return Err(format!(
                "expected window {}, but current window is {}",
                src.window, self.window.state.id
            ));
        }

        if self.window.slots.len() >= src.slot {
            return Err(format!(
                "overflow window slot, maximum slot is {}, but try indexing {}",
                self.window.slots.len(),
                src.slot
            ));
        }

        Ok(())
    }

    pub(super) fn get_slot(&self, src: Addr) -> Result<&Option<WindowSlot>, String> {
        self.check_slot(&src)?;

        Ok(self.window.slots.get(src.slot).unwrap())
    }

    pub(super) fn get_slot_mut(&mut self, src: Addr) -> Result<&mut Option<WindowSlot>, String> {
        self.check_slot(&src)?;

        Ok(Rc::get_mut(&mut self.window)
            .unwrap()
            .slots
            .get_mut(src.slot)
            .unwrap())
    }

    pub(super) fn enter_window(&mut self) -> Result<(), String> {
        let window = Rc::new(Window::new(self.window_stack.len()));
        self.window_stack.push(window.clone());
        self.window = window;

        Ok(())
    }

    pub(super) fn exit_window(&mut self) -> Result<(), String> {
        self.window_stack.pop();
        if let Some(prev_window) = self.window_stack.last().cloned() {
            self.window = prev_window;
            Ok(())
        } else {
            Err("can not exit window, prev window is empty".into())
        }
    }
}
