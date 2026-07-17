use std::{collections::HashMap, rc::Rc};

use graftvm_data::data::Liternal;
use graftvm_error::RuntimeError;
use graftvm_window::Window;

use crate::opcode::Opcode;

mod constant;
mod primitive;
mod window;

pub type Bytecode = Vec<Opcode>;

#[derive(Default)]
struct VMFlags {
    pub cmp: bool,
}

pub struct VM {
    window: Rc<Window>,
    window_stack: Vec<Rc<Window>>,
    constant_pool: HashMap<usize, Rc<Liternal>>,
    function_pool: HashMap<usize, Bytecode>,
    flags: VMFlags,
}

impl VM {
    pub fn new(function_pool: HashMap<usize, Bytecode>) -> Self {
        let bottom_window = Rc::new(Window::new(0));

        Self {
            window: bottom_window.clone(),
            window_stack: vec![bottom_window],
            constant_pool: HashMap::new(),
            function_pool,
            flags: VMFlags::default(),
        }
    }

    pub fn execute(&mut self, opcode: Opcode) -> Result<(), RuntimeError> {
        match opcode {
            Opcode::Add { dst, lhs, rhs } => {}

            Opcode::StoreData { index, data } => self.store_data(index, data),
            Opcode::LoadData { dst, index } => self.load_data(dst, index)?,

            Opcode::Move { dst, src } => *self.get_slot_mut(dst)? = self.get_slot_mut(src)?.take(),
            Opcode::Copy { dst, src } => *self.get_slot_mut(dst)? = self.get_slot_mut(src)?.clone(),
            Opcode::Drop { src } => *self.get_slot_mut(src)? = None,

            Opcode::Call { dst, index, arg } => {
                if let Some(bytecode) = self.function_pool.get(&index).cloned() {
                    self.enter_window()?;

                    *self.get_arg_slot_mut()? = self.get_slot_mut(arg)?.clone();

                    self.run(bytecode)?;

                    *self.get_slot_mut(dst)? = self.get_ret_slot_mut()?.take();

                    self.exit_window()?;
                } else {
                    return Err(RuntimeError::new(format!(
                        "function pool index {index} is empty"
                    )));
                }
            }

            Opcode::Enter => {
                self.enter_window()?;
            }
            Opcode::Exit => {
                self.exit_window()?;
            }
        }

        Ok(())
    }

    pub fn run(&mut self, bytecode: Bytecode) -> Result<(), RuntimeError> {
        for opcode in bytecode {
            self.execute(opcode)?;
        }

        Ok(())
    }
}
