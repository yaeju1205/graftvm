use std::{collections::HashMap, rc::Rc};

use graftvm_bytecode::{Bytecode, Opcode, Width};
use graftvm_liternal::Liternal;
use graftvm_window::Window;

mod arithmetic;
mod bitwise;
mod compare;
mod constant;
mod conversion;
mod unary;
mod utils;
mod window;

#[derive(Default)]
struct VMState {
    pub cmp: bool,
}

pub struct VM {
    bytecode: Bytecode,
    pc: usize,
    window: Rc<Window>,
    window_stack: Vec<Rc<Window>>,
    constant_pool: HashMap<usize, Rc<Liternal>>,
    state: VMState,
}

// ── width-dispatch helpers used across modules ──

macro_rules! binop_width {
    ($self:expr, $dst:expr, $lhs:expr, $rhs:expr, $ty:expr, |$v:ident, $r:ident| $expr:expr) => {{
        let (lhs_frag, rhs_frag) = $self.expect_number_lhs_rhs($lhs, $rhs)?;
        match $ty {
            Width::I8 => {
                let $v = lhs_frag.data.expect_int()?.expect_i8()?;
                let $r = rhs_frag.data.expect_int()?.expect_i8()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::I16 => {
                let $v = lhs_frag.data.expect_int()?.expect_i16()?;
                let $r = rhs_frag.data.expect_int()?.expect_i16()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::I32 => {
                let $v = lhs_frag.data.expect_int()?.expect_i32()?;
                let $r = rhs_frag.data.expect_int()?.expect_i32()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::I64 => {
                let $v = lhs_frag.data.expect_int()?.expect_i64()?;
                let $r = rhs_frag.data.expect_int()?.expect_i64()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::U8 => {
                let $v = lhs_frag.data.expect_uint()?.expect_u8()?;
                let $r = rhs_frag.data.expect_uint()?.expect_u8()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::U16 => {
                let $v = lhs_frag.data.expect_uint()?.expect_u16()?;
                let $r = rhs_frag.data.expect_uint()?.expect_u16()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::U32 => {
                let $v = lhs_frag.data.expect_uint()?.expect_u32()?;
                let $r = rhs_frag.data.expect_uint()?.expect_u32()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::U64 => {
                let $v = lhs_frag.data.expect_uint()?.expect_u64()?;
                let $r = rhs_frag.data.expect_uint()?.expect_u64()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::F32 => {
                let $v = lhs_frag.data.expect_float()?.expect_f32()?;
                let $r = rhs_frag.data.expect_float()?.expect_f32()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::F64 => {
                let $v = lhs_frag.data.expect_float()?.expect_f64()?;
                let $r = rhs_frag.data.expect_float()?.expect_f64()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
        }
        Ok(())
    }};
}

macro_rules! unop_width {
    ($self:expr, $dst:expr, $src:expr, $ty:expr, |$v:ident| $expr:expr) => {{
        let frag = $self.expect_number($src)?;
        match $ty {
            Width::I8 => {
                let $v = frag.data.expect_int()?.expect_i8()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::I16 => {
                let $v = frag.data.expect_int()?.expect_i16()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::I32 => {
                let $v = frag.data.expect_int()?.expect_i32()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::I64 => {
                let $v = frag.data.expect_int()?.expect_i64()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::U8 => {
                let $v = frag.data.expect_uint()?.expect_u8()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::U16 => {
                let $v = frag.data.expect_uint()?.expect_u16()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::U32 => {
                let $v = frag.data.expect_uint()?.expect_u32()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::U64 => {
                let $v = frag.data.expect_uint()?.expect_u64()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::F32 => {
                let $v = frag.data.expect_float()?.expect_f32()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
            Width::F64 => {
                let $v = frag.data.expect_float()?.expect_f64()?;
                *$self.get_slot_mut($dst)? = Some($crate::vm::pipe_ws_lit!($expr));
            }
        }
        Ok(())
    }};
}

#[macro_export]
macro_rules! pipe_ws_lit {
    ($expr:expr) => {
        rpipe::pipe!(graftvm_window::WindowSlot::from, graftvm_liternal::Liternal::from, $expr)
    };
}

#[macro_export]
macro_rules! pipe_ws_lit_some {
    ($expr:expr) => {
        rpipe::pipe!(Some, graftvm_window::WindowSlot::from, graftvm_liternal::Liternal::from, $expr)
    };
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        let bottom_window = Rc::new(Window::new(0));

        Self {
            bytecode,
            pc: 0,
            window: bottom_window.clone(),
            window_stack: vec![bottom_window],
            constant_pool: HashMap::new(),
            state: VMState::default(),
        }
    }

    fn execute(&mut self, opcode: Opcode) -> Result<(), String> {
        match opcode {
            Opcode::Add { dst, lhs, rhs, ty } => self.add(dst, lhs, rhs, ty)?,
            Opcode::Sub { dst, lhs, rhs, ty } => self.sub(dst, lhs, rhs, ty)?,
            Opcode::Mul { dst, lhs, rhs, ty } => self.mul(dst, lhs, rhs, ty)?,
            Opcode::Div { dst, lhs, rhs, ty } => self.div(dst, lhs, rhs, ty)?,
            Opcode::Rem { dst, lhs, rhs, ty } => self.rem(dst, lhs, rhs, ty)?,
            Opcode::Neg { dst, src, ty } => self.neg(dst, src, ty)?,

            Opcode::Lt { dst, lhs, rhs, ty } => self.lt(dst, lhs, rhs, ty)?,
            Opcode::Le { dst, lhs, rhs, ty } => self.le(dst, lhs, rhs, ty)?,
            Opcode::Gt { dst, lhs, rhs, ty } => self.gt(dst, lhs, rhs, ty)?,
            Opcode::Ge { dst, lhs, rhs, ty } => self.ge(dst, lhs, rhs, ty)?,
            Opcode::Eq { lhs, rhs } => self.eq(lhs, rhs)?,
            Opcode::Neq { lhs, rhs } => self.neq(lhs, rhs)?,

            Opcode::And { dst, lhs, rhs, ty } => self.and(dst, lhs, rhs, ty)?,
            Opcode::Or { dst, lhs, rhs, ty } => self.or(dst, lhs, rhs, ty)?,
            Opcode::Xor { dst, lhs, rhs, ty } => self.xor(dst, lhs, rhs, ty)?,
            Opcode::Not { dst, src, ty } => self.not(dst, src, ty)?,

            Opcode::Shl { dst, lhs, rhs, ty } => self.shl(dst, lhs, rhs, ty)?,
            Opcode::Shr { dst, lhs, rhs, ty } => self.shr(dst, lhs, rhs, ty)?,

            Opcode::Extend { dst, src, signed } => self.extend(dst, src, signed)?,
            Opcode::Trunc { dst, src } => self.trunc(dst, src)?,
            Opcode::Reinterpret { dst, src } => self.reinterpret(dst, src)?,
            Opcode::Convert { dst, src, signed } => self.convert(dst, src, signed)?,

            Opcode::Jump(pc) => {
                self.pc = pc;
                return Ok(());
            }
            Opcode::Branch(pc) => {
                if self.state.cmp {
                    self.pc = pc;
                }
                return Ok(());
            }

            Opcode::StoreData { index, data } => self.store_data(index, data),
            Opcode::LoadData { dst, index } => self.load_data(dst, index)?,

            Opcode::Move { dst, src } => {
                *self.get_slot_mut(dst)? = self.get_slot_mut(src)?.take();
            }
            Opcode::Copy { dst, src } => {
                *self.get_slot_mut(dst)? = self.get_slot_mut(src)?.clone();
            }
            Opcode::Drop { src } => {
                *self.get_slot_mut(src)? = None;
            }

            Opcode::Enter => self.enter_window()?,
            Opcode::Exit => self.exit_window()?,
        }

        self.pc += 1;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), String> {
        let opcode = self.bytecode[self.pc].clone();
        self.execute(opcode)
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.pc < self.bytecode.len() {
            self.step()?;
        }
        Ok(())
    }
}
