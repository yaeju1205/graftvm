use graftvm_bytecode::Addr;
use graftvm_liternal::{Float, Int, Liternal, UInt};
use graftvm_window::WindowSlot;

use crate::vm::VM;

#[derive(Clone, Copy)]
enum Cmp { Lt, Le, Gt, Ge }

impl VM {
    fn cmp(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width, op: Cmp) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        let result = match (ty, op) {
            (Width::I8, Cmp::Lt) => Int::Int8(vl.expect_int()?.expect_i8()?) < Int::Int8(vr.expect_int()?.expect_i8()?),
            (Width::I8, Cmp::Le) => Int::Int8(vl.expect_int()?.expect_i8()?) <= Int::Int8(vr.expect_int()?.expect_i8()?),
            (Width::I8, Cmp::Gt) => Int::Int8(vl.expect_int()?.expect_i8()?) > Int::Int8(vr.expect_int()?.expect_i8()?),
            (Width::I8, Cmp::Ge) => Int::Int8(vl.expect_int()?.expect_i8()?) >= Int::Int8(vr.expect_int()?.expect_i8()?),

            (Width::I16, Cmp::Lt) => Int::Int16(vl.expect_int()?.expect_i16()?) < Int::Int16(vr.expect_int()?.expect_i16()?),
            (Width::I16, Cmp::Le) => Int::Int16(vl.expect_int()?.expect_i16()?) <= Int::Int16(vr.expect_int()?.expect_i16()?),
            (Width::I16, Cmp::Gt) => Int::Int16(vl.expect_int()?.expect_i16()?) > Int::Int16(vr.expect_int()?.expect_i16()?),
            (Width::I16, Cmp::Ge) => Int::Int16(vl.expect_int()?.expect_i16()?) >= Int::Int16(vr.expect_int()?.expect_i16()?),

            (Width::I32, Cmp::Lt) => Int::Int32(vl.expect_int()?.expect_i32()?) < Int::Int32(vr.expect_int()?.expect_i32()?),
            (Width::I32, Cmp::Le) => Int::Int32(vl.expect_int()?.expect_i32()?) <= Int::Int32(vr.expect_int()?.expect_i32()?),
            (Width::I32, Cmp::Gt) => Int::Int32(vl.expect_int()?.expect_i32()?) > Int::Int32(vr.expect_int()?.expect_i32()?),
            (Width::I32, Cmp::Ge) => Int::Int32(vl.expect_int()?.expect_i32()?) >= Int::Int32(vr.expect_int()?.expect_i32()?),

            (Width::I64, Cmp::Lt) => Int::Int64(vl.expect_int()?.expect_i64()?) < Int::Int64(vr.expect_int()?.expect_i64()?),
            (Width::I64, Cmp::Le) => Int::Int64(vl.expect_int()?.expect_i64()?) <= Int::Int64(vr.expect_int()?.expect_i64()?),
            (Width::I64, Cmp::Gt) => Int::Int64(vl.expect_int()?.expect_i64()?) > Int::Int64(vr.expect_int()?.expect_i64()?),
            (Width::I64, Cmp::Ge) => Int::Int64(vl.expect_int()?.expect_i64()?) >= Int::Int64(vr.expect_int()?.expect_i64()?),

            (Width::U8, Cmp::Lt) => vr.expect_uint()?.expect_u8()? < vr.expect_uint()?.expect_u8()?,
            // Hmm, wait, that's using vr twice. Let me rewrite this differently.
        }
    }
}
```

This is getting ugly. Let me just go back to the simple approach of 4 separate functions with match arms returning bool.

Actually, the simplest working approach: just have four functions, each with a match. No macro. Just direct code.

Wait, I realize I keep going back and forth. Let me just write clean code. A single helper with a 40-arm match (4 ops × 10 widths) is fine. The match is exhaustive and LLVM compiles it efficiently.<｜end▁of▁thinking｜>

<｜｜DSML｜｜tool_calls>
<｜｜DSML｜｜invoke name="computer_file_write">
<｜｜DSML｜｜parameter name="content" string="true">use graftvm_bytecode::{Addr, Width};
use graftvm_liternal::Liternal;
use graftvm_window::WindowSlot;

use crate::vm::VM;

impl VM {
    pub(crate) fn lt(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        let ok = match ty {
            Width::I8 => vl.expect_int()?.expect_i8()? < vr.expect_int()?.expect_i8()?,
            Width::I16 => vl.expect_int()?.expect_i16()? < vr.expect_int()?.expect_i16()?,
            Width::I32 => vl.expect_int()?.expect_i32()? < vr.expect_int()?.expect_i32()?,
            Width::I64 => vl.expect_int()?.expect_i64()? < vr.expect_int()?.expect_i64()?,
            Width::U8 => vl.expect_uint()?.expect_u8()? < vr.expect_uint()?.expect_u8()?,
            Width::U16 => vl.expect_uint()?.expect_u16()? < vr.expect_uint()?.expect_u16()?,
            Width::U32 => vl.expect_uint()?.expect_u32()? < vr.expect_uint()?.expect_u32()?,
            Width::U64 => vl.expect_uint()?.expect_u64()? < vr.expect_uint()?.expect_u64()?,
            Width::F32 => vl.expect_float()?.expect_f32()? < vr.expect_float()?.expect_f32()?,
            Width::F64 => vl.expect_float()?.expect_f64()? < vr.expect_float()?.expect_f64()?,
        };
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(Liternal::from(ok)));
        Ok(())
    }

    pub(crate) fn le(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        let ok = match ty {
            Width::I8 => vl.expect_int()?.expect_i8()? <= vr.expect_int()?.expect_i8()?,
            Width::I16 => vl.expect_int()?.expect_i16()? <= vr.expect_int()?.expect_i16()?,
            Width::I32 => vl.expect_int()?.expect_i32()? <= vr.expect_int()?.expect_i32()?,
            Width::I64 => vl.expect_int()?.expect_i64()? <= vr.expect_int()?.expect_i64()?,
            Width::U8 => vl.expect_uint()?.expect_u8()? <= vr.expect_uint()?.expect_u8()?,
            Width::U16 => vl.expect_uint()?.expect_u16()? <= vr.expect_uint()?.expect_u16()?,
            Width::U32 => vl.expect_uint()?.expect_u32()? <= vr.expect_uint()?.expect_u32()?,
            Width::U64 => vl.expect_uint()?.expect_u64()? <= vr.expect_uint()?.expect_u64()?,
            Width::F32 => vl.expect_float()?.expect_f32()? <= vr.expect_float()?.expect_f32()?,
            Width::F64 => vl.expect_float()?.expect_f64()? <= vr.expect_float()?.expect_f64()?,
        };
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(Liternal::from(ok)));
        Ok(())
    }

    pub(crate) fn gt(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        let ok = match ty {
            Width::I8 => vl.expect_int()?.expect_i8()? > vr.expect_int()?.expect_i8()?,
            Width::I16 => vl.expect_int()?.expect_i16()? > vr.expect_int()?.expect_i16()?,
            Width::I32 => vl.expect_int()?.expect_i32()? > vr.expect_int()?.expect_i32()?,
            Width::I64 => vl.expect_int()?.expect_i64()? > vr.expect_int()?.expect_i64()?,
            Width::U8 => vl.expect_uint()?.expect_u8()? > vr.expect_uint()?.expect_u8()?,
            Width::U16 => vl.expect_uint()?.expect_u16()? > vr.expect_uint()?.expect_u16()?,
            Width::U32 => vl.expect_uint()?.expect_u32()? > vr.expect_uint()?.expect_u32()?,
            Width::U64 => vl.expect_uint()?.expect_u64()? > vr.expect_uint()?.expect_u64()?,
            Width::F32 => vl.expect_float()?.expect_f32()? > vr.expect_float()?.expect_f32()?,
            Width::F64 => vl.expect_float()?.expect_f64()? > vr.expect_float()?.expect_f64()?,
        };
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(Liternal::from(ok)));
        Ok(())
    }

    pub(crate) fn ge(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        let ok = match ty {
            Width::I8 => vl.expect_int()?.expect_i8()? >= vr.expect_int()?.expect_i8()?,
            Width::I16 => vl.expect_int()?.expect_i16()? >= vr.expect_int()?.expect_i16()?,
            Width::I32 => vl.expect_int()?.expect_i32()? >= vr.expect_int()?.expect_i32()?,
            Width::I64 => vl.expect_int()?.expect_i64()? >= vr.expect_int()?.expect_i64()?,
            Width::U8 => vl.expect_uint()?.expect_u8()? >= vr.expect_uint()?.expect_u8()?,
            Width::U16 => vl.expect_uint()?.expect_u16()? >= vr.expect_uint()?.expect_u16()?,
            Width::U32 => vl.expect_uint()?.expect_u32()? >= vr.expect_uint()?.expect_u32()?,
            Width::U64 => vl.expect_uint()?.expect_u64()? >= vr.expect_uint()?.expect_u64()?,
            Width::F32 => vl.expect_float()?.expect_f32()? >= vr.expect_float()?.expect_f32()?,
            Width::F64 => vl.expect_float()?.expect_f64()? >= vr.expect_float()?.expect_f64()?,
        };
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(Liternal::from(ok)));
        Ok(())
    }

    pub(crate) fn eq(&mut self, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        self.state.cmp = vl == vr;
        Ok(())
    }

    pub(crate) fn neq(&mut self, lhs: Addr, rhs: Addr) -> Result<(), String> {
        self.eq(lhs, rhs)?;
        self.state.cmp = !self.state.cmp;
        Ok(())
    }
}
