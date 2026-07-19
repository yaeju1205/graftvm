use std::{collections::HashMap, rc::Rc};

use graftvm_liternal::Liternal;
use graftvm_window::Window;

use crate::opcode::Opcode;

mod arithmetic;
mod bitwise;
mod compare;
mod constant;
mod conversion;
mod unary;
mod utils;
mod window;

pub type Bytecode = Vec<Opcode>;

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
            Opcode::AddI8 { dst, lhs, rhs } => self.addi8(dst, lhs, rhs)?,
            Opcode::AddI16 { dst, lhs, rhs } => self.addi16(dst, lhs, rhs)?,
            Opcode::AddI32 { dst, lhs, rhs } => self.addi32(dst, lhs, rhs)?,
            Opcode::AddI64 { dst, lhs, rhs } => self.addi64(dst, lhs, rhs)?,

            Opcode::SubI8 { dst, lhs, rhs } => self.subi8(dst, lhs, rhs)?,
            Opcode::SubI16 { dst, lhs, rhs } => self.subi16(dst, lhs, rhs)?,
            Opcode::SubI32 { dst, lhs, rhs } => self.subi32(dst, lhs, rhs)?,
            Opcode::SubI64 { dst, lhs, rhs } => self.subi64(dst, lhs, rhs)?,

            Opcode::MulI8 { dst, lhs, rhs } => self.muli8(dst, lhs, rhs)?,
            Opcode::MulI16 { dst, lhs, rhs } => self.muli16(dst, lhs, rhs)?,
            Opcode::MulI32 { dst, lhs, rhs } => self.muli32(dst, lhs, rhs)?,
            Opcode::MulI64 { dst, lhs, rhs } => self.muli64(dst, lhs, rhs)?,

            Opcode::DivI8 { dst, lhs, rhs } => self.divi8(dst, lhs, rhs)?,
            Opcode::DivI16 { dst, lhs, rhs } => self.divi16(dst, lhs, rhs)?,
            Opcode::DivI32 { dst, lhs, rhs } => self.divi32(dst, lhs, rhs)?,
            Opcode::DivI64 { dst, lhs, rhs } => self.divi64(dst, lhs, rhs)?,

            Opcode::RemI8 { dst, lhs, rhs } => self.remi8(dst, lhs, rhs)?,
            Opcode::RemI16 { dst, lhs, rhs } => self.remi16(dst, lhs, rhs)?,
            Opcode::RemI32 { dst, lhs, rhs } => self.remi32(dst, lhs, rhs)?,
            Opcode::RemI64 { dst, lhs, rhs } => self.remi64(dst, lhs, rhs)?,

            Opcode::AddU8 { dst, lhs, rhs } => self.addu8(dst, lhs, rhs)?,
            Opcode::AddU16 { dst, lhs, rhs } => self.addu16(dst, lhs, rhs)?,
            Opcode::AddU32 { dst, lhs, rhs } => self.addu32(dst, lhs, rhs)?,
            Opcode::AddU64 { dst, lhs, rhs } => self.addu64(dst, lhs, rhs)?,

            Opcode::SubU8 { dst, lhs, rhs } => self.subu8(dst, lhs, rhs)?,
            Opcode::SubU16 { dst, lhs, rhs } => self.subu16(dst, lhs, rhs)?,
            Opcode::SubU32 { dst, lhs, rhs } => self.subu32(dst, lhs, rhs)?,
            Opcode::SubU64 { dst, lhs, rhs } => self.subu64(dst, lhs, rhs)?,

            Opcode::MulU8 { dst, lhs, rhs } => self.mulu8(dst, lhs, rhs)?,
            Opcode::MulU16 { dst, lhs, rhs } => self.mulu16(dst, lhs, rhs)?,
            Opcode::MulU32 { dst, lhs, rhs } => self.mulu32(dst, lhs, rhs)?,
            Opcode::MulU64 { dst, lhs, rhs } => self.mulu64(dst, lhs, rhs)?,

            Opcode::DivU8 { dst, lhs, rhs } => self.divu8(dst, lhs, rhs)?,
            Opcode::DivU16 { dst, lhs, rhs } => self.divu16(dst, lhs, rhs)?,
            Opcode::DivU32 { dst, lhs, rhs } => self.divu32(dst, lhs, rhs)?,
            Opcode::DivU64 { dst, lhs, rhs } => self.divu64(dst, lhs, rhs)?,

            Opcode::RemU8 { dst, lhs, rhs } => self.remu8(dst, lhs, rhs)?,
            Opcode::RemU16 { dst, lhs, rhs } => self.remu16(dst, lhs, rhs)?,
            Opcode::RemU32 { dst, lhs, rhs } => self.remu32(dst, lhs, rhs)?,
            Opcode::RemU64 { dst, lhs, rhs } => self.remu64(dst, lhs, rhs)?,

            Opcode::AddF32 { dst, lhs, rhs } => self.addf32(dst, lhs, rhs)?,
            Opcode::AddF64 { dst, lhs, rhs } => self.addf64(dst, lhs, rhs)?,

            Opcode::SubF32 { dst, lhs, rhs } => self.subf32(dst, lhs, rhs)?,
            Opcode::SubF64 { dst, lhs, rhs } => self.subf64(dst, lhs, rhs)?,

            Opcode::MulF32 { dst, lhs, rhs } => self.mulf32(dst, lhs, rhs)?,
            Opcode::MulF64 { dst, lhs, rhs } => self.mulf64(dst, lhs, rhs)?,

            Opcode::DivF32 { dst, lhs, rhs } => self.divf32(dst, lhs, rhs)?,
            Opcode::DivF64 { dst, lhs, rhs } => self.divf64(dst, lhs, rhs)?,

            Opcode::RemF32 { dst, lhs, rhs } => self.remf32(dst, lhs, rhs)?,
            Opcode::RemF64 { dst, lhs, rhs } => self.remf64(dst, lhs, rhs)?,

            Opcode::NegI8 { dst, src } => self.negi8(dst, src)?,
            Opcode::NegI16 { dst, src } => self.negi16(dst, src)?,
            Opcode::NegI32 { dst, src } => self.negi32(dst, src)?,
            Opcode::NegI64 { dst, src } => self.negi64(dst, src)?,

            Opcode::NegF32 { dst, src } => self.negf32(dst, src)?,
            Opcode::NegF64 { dst, src } => self.negf64(dst, src)?,

            Opcode::AndI8 { dst, lhs, rhs } => self.andi8(dst, lhs, rhs)?,
            Opcode::AndI16 { dst, lhs, rhs } => self.andi16(dst, lhs, rhs)?,
            Opcode::AndI32 { dst, lhs, rhs } => self.andi32(dst, lhs, rhs)?,
            Opcode::AndI64 { dst, lhs, rhs } => self.andi64(dst, lhs, rhs)?,

            Opcode::OrI8 { dst, lhs, rhs } => self.ori8(dst, lhs, rhs)?,
            Opcode::OrI16 { dst, lhs, rhs } => self.ori16(dst, lhs, rhs)?,
            Opcode::OrI32 { dst, lhs, rhs } => self.ori32(dst, lhs, rhs)?,
            Opcode::OrI64 { dst, lhs, rhs } => self.ori64(dst, lhs, rhs)?,

            Opcode::XorI8 { dst, lhs, rhs } => self.xori8(dst, lhs, rhs)?,
            Opcode::XorI16 { dst, lhs, rhs } => self.xori16(dst, lhs, rhs)?,
            Opcode::XorI32 { dst, lhs, rhs } => self.xori32(dst, lhs, rhs)?,
            Opcode::XorI64 { dst, lhs, rhs } => self.xori64(dst, lhs, rhs)?,

            Opcode::NotI8 { dst, src } => self.noti8(dst, src)?,
            Opcode::NotI16 { dst, src } => self.noti16(dst, src)?,
            Opcode::NotI32 { dst, src } => self.noti32(dst, src)?,
            Opcode::NotI64 { dst, src } => self.noti64(dst, src)?,

            Opcode::AndU8 { dst, lhs, rhs } => self.andu8(dst, lhs, rhs)?,
            Opcode::AndU16 { dst, lhs, rhs } => self.andu16(dst, lhs, rhs)?,
            Opcode::AndU32 { dst, lhs, rhs } => self.andu32(dst, lhs, rhs)?,
            Opcode::AndU64 { dst, lhs, rhs } => self.andu64(dst, lhs, rhs)?,

            Opcode::OrU8 { dst, lhs, rhs } => self.oru8(dst, lhs, rhs)?,
            Opcode::OrU16 { dst, lhs, rhs } => self.oru16(dst, lhs, rhs)?,
            Opcode::OrU32 { dst, lhs, rhs } => self.oru32(dst, lhs, rhs)?,
            Opcode::OrU64 { dst, lhs, rhs } => self.oru64(dst, lhs, rhs)?,

            Opcode::XorU8 { dst, lhs, rhs } => self.xoru8(dst, lhs, rhs)?,
            Opcode::XorU16 { dst, lhs, rhs } => self.xoru16(dst, lhs, rhs)?,
            Opcode::XorU32 { dst, lhs, rhs } => self.xoru32(dst, lhs, rhs)?,
            Opcode::XorU64 { dst, lhs, rhs } => self.xoru64(dst, lhs, rhs)?,

            Opcode::NotU8 { dst, src } => self.notu8(dst, src)?,
            Opcode::NotU16 { dst, src } => self.notu16(dst, src)?,
            Opcode::NotU32 { dst, src } => self.notu32(dst, src)?,
            Opcode::NotU64 { dst, src } => self.notu64(dst, src)?,

            Opcode::ShlI8 { dst, lhs, rhs } => self.shli8(dst, lhs, rhs)?,
            Opcode::ShlI16 { dst, lhs, rhs } => self.shli16(dst, lhs, rhs)?,
            Opcode::ShlI32 { dst, lhs, rhs } => self.shli32(dst, lhs, rhs)?,
            Opcode::ShlI64 { dst, lhs, rhs } => self.shli64(dst, lhs, rhs)?,

            Opcode::ShlU8 { dst, lhs, rhs } => self.shlu8(dst, lhs, rhs)?,
            Opcode::ShlU16 { dst, lhs, rhs } => self.shlu16(dst, lhs, rhs)?,
            Opcode::ShlU32 { dst, lhs, rhs } => self.shlu32(dst, lhs, rhs)?,
            Opcode::ShlU64 { dst, lhs, rhs } => self.shlu64(dst, lhs, rhs)?,

            Opcode::ShrI8 { dst, lhs, rhs } => self.shri8(dst, lhs, rhs)?,
            Opcode::ShrI16 { dst, lhs, rhs } => self.shri16(dst, lhs, rhs)?,
            Opcode::ShrI32 { dst, lhs, rhs } => self.shri32(dst, lhs, rhs)?,
            Opcode::ShrI64 { dst, lhs, rhs } => self.shri64(dst, lhs, rhs)?,

            Opcode::ShrU8 { dst, lhs, rhs } => self.shru8(dst, lhs, rhs)?,
            Opcode::ShrU16 { dst, lhs, rhs } => self.shru16(dst, lhs, rhs)?,
            Opcode::ShrU32 { dst, lhs, rhs } => self.shru32(dst, lhs, rhs)?,
            Opcode::ShrU64 { dst, lhs, rhs } => self.shru64(dst, lhs, rhs)?,

            Opcode::LtI8 { dst, lhs, rhs } => self.lti8(dst, lhs, rhs)?,
            Opcode::LtI16 { dst, lhs, rhs } => self.lti16(dst, lhs, rhs)?,
            Opcode::LtI32 { dst, lhs, rhs } => self.lti32(dst, lhs, rhs)?,
            Opcode::LtI64 { dst, lhs, rhs } => self.lti64(dst, lhs, rhs)?,
            Opcode::LtU8 { dst, lhs, rhs } => self.ltu8(dst, lhs, rhs)?,
            Opcode::LtU16 { dst, lhs, rhs } => self.ltu16(dst, lhs, rhs)?,
            Opcode::LtU32 { dst, lhs, rhs } => self.ltu32(dst, lhs, rhs)?,
            Opcode::LtU64 { dst, lhs, rhs } => self.ltu64(dst, lhs, rhs)?,
            Opcode::LtF32 { dst, lhs, rhs } => self.ltf32(dst, lhs, rhs)?,
            Opcode::LtF64 { dst, lhs, rhs } => self.ltf64(dst, lhs, rhs)?,

            Opcode::LeI8 { dst, lhs, rhs } => self.lei8(dst, lhs, rhs)?,
            Opcode::LeI16 { dst, lhs, rhs } => self.lei16(dst, lhs, rhs)?,
            Opcode::LeI32 { dst, lhs, rhs } => self.lei32(dst, lhs, rhs)?,
            Opcode::LeI64 { dst, lhs, rhs } => self.lei64(dst, lhs, rhs)?,
            Opcode::LeU8 { dst, lhs, rhs } => self.leu8(dst, lhs, rhs)?,
            Opcode::LeU16 { dst, lhs, rhs } => self.leu16(dst, lhs, rhs)?,
            Opcode::LeU32 { dst, lhs, rhs } => self.leu32(dst, lhs, rhs)?,
            Opcode::LeU64 { dst, lhs, rhs } => self.leu64(dst, lhs, rhs)?,
            Opcode::LeF32 { dst, lhs, rhs } => self.lef32(dst, lhs, rhs)?,
            Opcode::LeF64 { dst, lhs, rhs } => self.lef64(dst, lhs, rhs)?,

            Opcode::GtI8 { dst, lhs, rhs } => self.gti8(dst, lhs, rhs)?,
            Opcode::GtI16 { dst, lhs, rhs } => self.gti16(dst, lhs, rhs)?,
            Opcode::GtI32 { dst, lhs, rhs } => self.gti32(dst, lhs, rhs)?,
            Opcode::GtI64 { dst, lhs, rhs } => self.gti64(dst, lhs, rhs)?,
            Opcode::GtU8 { dst, lhs, rhs } => self.gtu8(dst, lhs, rhs)?,
            Opcode::GtU16 { dst, lhs, rhs } => self.gtu16(dst, lhs, rhs)?,
            Opcode::GtU32 { dst, lhs, rhs } => self.gtu32(dst, lhs, rhs)?,
            Opcode::GtU64 { dst, lhs, rhs } => self.gtu64(dst, lhs, rhs)?,
            Opcode::GtF32 { dst, lhs, rhs } => self.gtf32(dst, lhs, rhs)?,
            Opcode::GtF64 { dst, lhs, rhs } => self.gtf64(dst, lhs, rhs)?,

            Opcode::GeI8 { dst, lhs, rhs } => self.gei8(dst, lhs, rhs)?,
            Opcode::GeI16 { dst, lhs, rhs } => self.gei16(dst, lhs, rhs)?,
            Opcode::GeI32 { dst, lhs, rhs } => self.gei32(dst, lhs, rhs)?,
            Opcode::GeI64 { dst, lhs, rhs } => self.gei64(dst, lhs, rhs)?,
            Opcode::GeU8 { dst, lhs, rhs } => self.geu8(dst, lhs, rhs)?,
            Opcode::GeU16 { dst, lhs, rhs } => self.geu16(dst, lhs, rhs)?,
            Opcode::GeU32 { dst, lhs, rhs } => self.geu32(dst, lhs, rhs)?,
            Opcode::GeU64 { dst, lhs, rhs } => self.geu64(dst, lhs, rhs)?,
            Opcode::GeF32 { dst, lhs, rhs } => self.gef32(dst, lhs, rhs)?,
            Opcode::GeF64 { dst, lhs, rhs } => self.gef64(dst, lhs, rhs)?,

            Opcode::ExtendI8I16 { dst, src } => self.extendi8_i16(dst, src)?,
            Opcode::ExtendI8I32 { dst, src } => self.extendi8_i32(dst, src)?,
            Opcode::ExtendI8I64 { dst, src } => self.extendi8_i64(dst, src)?,
            Opcode::ExtendI16I32 { dst, src } => self.extendi16_i32(dst, src)?,
            Opcode::ExtendI16I64 { dst, src } => self.extendi16_i64(dst, src)?,
            Opcode::ExtendI32I64 { dst, src } => self.extendi32_i64(dst, src)?,

            Opcode::ExtendU8U16 { dst, src } => self.extendu8_u16(dst, src)?,
            Opcode::ExtendU8U32 { dst, src } => self.extendu8_u32(dst, src)?,
            Opcode::ExtendU8U64 { dst, src } => self.extendu8_u64(dst, src)?,
            Opcode::ExtendU16U32 { dst, src } => self.extendu16_u32(dst, src)?,
            Opcode::ExtendU16U64 { dst, src } => self.extendu16_u64(dst, src)?,
            Opcode::ExtendU32U64 { dst, src } => self.extendu32_u64(dst, src)?,

            Opcode::TruncI64I32 { dst, src } => self.trunci64_i32(dst, src)?,
            Opcode::TruncI64I16 { dst, src } => self.trunci64_i16(dst, src)?,
            Opcode::TruncI64I8 { dst, src } => self.trunci64_i8(dst, src)?,
            Opcode::TruncI32I16 { dst, src } => self.trunci32_i16(dst, src)?,
            Opcode::TruncI32I8 { dst, src } => self.trunci32_i8(dst, src)?,
            Opcode::TruncI16I8 { dst, src } => self.trunci16_i8(dst, src)?,

            Opcode::ReinterpretU8I8 { dst, src } => self.reinterpu8_i8(dst, src)?,
            Opcode::ReinterpretU16I16 { dst, src } => self.reinterpu16_i16(dst, src)?,
            Opcode::ReinterpretU32I32 { dst, src } => self.reinterpu32_i32(dst, src)?,
            Opcode::ReinterpretU64I64 { dst, src } => self.reinterpu64_i64(dst, src)?,
            Opcode::ReinterpretI8U8 { dst, src } => self.reinterpi8_u8(dst, src)?,
            Opcode::ReinterpretI16U16 { dst, src } => self.reinterpi16_u16(dst, src)?,
            Opcode::ReinterpretI32U32 { dst, src } => self.reinterpi32_u32(dst, src)?,
            Opcode::ReinterpretI64U64 { dst, src } => self.reinterpi64_u64(dst, src)?,

            Opcode::IntToF32 { dst, src } => self.inttof32(dst, src)?,
            Opcode::IntToF64 { dst, src } => self.inttof64(dst, src)?,
            Opcode::UIntToF32 { dst, src } => self.uinttof32(dst, src)?,
            Opcode::UIntToF64 { dst, src } => self.uinttof64(dst, src)?,
            Opcode::FToI32 { dst, src } => self.ftoint32(dst, src)?,
            Opcode::FToI64 { dst, src } => self.ftoint64(dst, src)?,
            Opcode::FToU32 { dst, src } => self.ftouint32(dst, src)?,
            Opcode::FToU64 { dst, src } => self.ftouint64(dst, src)?,

            Opcode::Eq { lhs, rhs } => self.eq(lhs, rhs)?,
            Opcode::Neq { lhs, rhs } => self.neq(lhs, rhs)?,

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

            Opcode::Move { dst, src } => *self.get_slot_mut(dst)? = self.get_slot_mut(src)?.take(),
            Opcode::Copy { dst, src } => *self.get_slot_mut(dst)? = self.get_slot_mut(src)?.clone(),
            Opcode::Drop { src } => *self.get_slot_mut(src)? = None,

            Opcode::Enter => {
                self.enter_window()?;
            }
            Opcode::Exit => {
                self.exit_window()?;
            }
        }

        self.pc += 1;

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), String> {
        let opcode = self.bytecode[self.pc].clone();

        self.pc += 1;

        self.execute(opcode)
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.pc < self.bytecode.len() {
            self.step()?;
        }

        Ok(())
    }
}
