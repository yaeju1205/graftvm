use std::rc::Rc;

use graftvm_error::RuntimeError;
use graftvm_fragment::Fragment;
use graftvm_window::Window;

use crate::{opcode::Addr, vm::VM};

impl VM {
    pub(super) fn get_ret_slot_mut(&mut self) -> Result<&mut Option<Fragment>, RuntimeError> {
        self.get_slot_mut(Addr {
            window: self.window.state.id,
            slot: 0,
        })
    }

    pub(super) fn get_arg_slot_mut(&mut self) -> Result<&mut Option<Fragment>, RuntimeError> {
        self.get_slot_mut(Addr {
            window: self.window.state.id,
            slot: 1,
        })
    }

    fn check_slot(&self, src: &Addr) -> Result<(), RuntimeError> {
        if self.window.state.id != src.window {
            return Err(RuntimeError::new(format!(
                "expected window {}, but current window is {}",
                src.window, self.window.state.id
            )));
        }

        if self.window.slots.len() >= src.slot {
            return Err(RuntimeError::new(format!(
                "overflow window slot, maximum slot is {}, but try indexing {}",
                self.window.slots.len(),
                src.slot
            )));
        }

        Ok(())
    }

    pub(super) fn get_slot(&self, src: Addr) -> Result<&Option<Fragment>, RuntimeError> {
        self.check_slot(&src)?;

        Ok(self.window.slots.get(src.slot).unwrap())
    }

    pub(super) fn get_slot_mut(
        &mut self,
        src: Addr,
    ) -> Result<&mut Option<Fragment>, RuntimeError> {
        self.check_slot(&src)?;

        Ok(Rc::get_mut(&mut self.window)
            .unwrap()
            .slots
            .get_mut(src.slot)
            .unwrap())
    }

    pub(super) fn enter_window(&mut self) -> Result<(), RuntimeError> {
        let window = Rc::new(Window::new(self.window_stack.len()));
        self.window_stack.push(window.clone());
        self.window = window;

        Ok(())
    }

    pub(super) fn exit_window(&mut self) -> Result<(), RuntimeError> {
        self.window_stack.pop();
        if let Some(prev_window) = self.window_stack.last().cloned() {
            self.window = prev_window;
            Ok(())
        } else {
            Err(RuntimeError::new(
                "can not exit window, prev window is empty".into(),
            ))
        }
    }
}
