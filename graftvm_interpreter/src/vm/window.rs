use graftvm_window::Window;

use crate::vm::VM;

impl VM {
    pub(super) fn enter_window(&mut self) -> Result<(), String> {
        let id = self.current_window().id + 1;
        self.window_stack.push(Window::new(id));
        Ok(())
    }

    pub(super) fn exit_window(&mut self) -> Result<(), String> {
        if self.window_stack.len() <= 1 {
            return Err("cannot exit the bottom window".into());
        }
        self.window_stack.pop();
        Ok(())
    }
}
