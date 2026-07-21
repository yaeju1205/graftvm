use graftvm_bytecode::{Addr, Width};
use graftvm_liternal::Liternal;
use graftvm_window::WindowSlot;

use crate::vm::{binop_width, VM};

impl VM {
    pub(crate) fn add(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        binop_width!(self, dst, lhs, rhs, ty, |v, r| v + r)
    }

    pub(crate) fn sub(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        binop_width!(self, dst, lhs, rhs, ty, |v, r| v - r)
    }

    pub(crate) fn mul(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        binop_width!(self, dst, lhs, rhs, ty, |v, r| v * r)
    }

    pub(crate) fn div(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        let result = match ty {
            Width::I8 => { let v = vl.expect_int()?.expect_i8()?; let r = vr.expect_int()?.expect_i8()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v / r) }
            Width::I16 => { let v = vl.expect_int()?.expect_i16()?; let r = vr.expect_int()?.expect_i16()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v / r) }
            Width::I32 => { let v = vl.expect_int()?.expect_i32()?; let r = vr.expect_int()?.expect_i32()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v / r) }
            Width::I64 => { let v = vl.expect_int()?.expect_i64()?; let r = vr.expect_int()?.expect_i64()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v / r) }
            Width::U8 => { let v = vl.expect_uint()?.expect_u8()?; let r = vr.expect_uint()?.expect_u8()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v / r) }
            Width::U16 => { let v = vl.expect_uint()?.expect_u16()?; let r = vr.expect_uint()?.expect_u16()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v / r) }
            Width::U32 => { let v = vl.expect_uint()?.expect_u32()?; let r = vr.expect_uint()?.expect_u32()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v / r) }
            Width::U64 => { let v = vl.expect_uint()?.expect_u64()?; let r = vr.expect_uint()?.expect_u64()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v / r) }
            Width::F32 => { let v = vl.expect_float()?.expect_f32()?; let r = vr.expect_float()?.expect_f32()?; if r == 0.0 { return Err("division by zero".into()) } Liternal::from(v / r) }
            Width::F64 => { let v = vl.expect_float()?.expect_f64()?; let r = vr.expect_float()?.expect_f64()?; if r == 0.0 { return Err("division by zero".into()) } Liternal::from(v / r) }
        };
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(result));
        Ok(())
    }

    pub(crate) fn rem(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        let result = match ty {
            Width::I8 => { let v = vl.expect_int()?.expect_i8()?; let r = vr.expect_int()?.expect_i8()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v % r) }
            Width::I16 => { let v = vl.expect_int()?.expect_i16()?; let r = vr.expect_int()?.expect_i16()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v % r) }
            Width::I32 => { let v = vl.expect_int()?.expect_i32()?; let r = vr.expect_int()?.expect_i32()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v % r) }
            Width::I64 => { let v = vl.expect_int()?.expect_i64()?; let r = vr.expect_int()?.expect_i64()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v % r) }
            Width::U8 => { let v = vl.expect_uint()?.expect_u8()?; let r = vr.expect_uint()?.expect_u8()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v % r) }
            Width::U16 => { let v = vl.expect_uint()?.expect_u16()?; let r = vr.expect_uint()?.expect_u16()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v % r) }
            Width::U32 => { let v = vl.expect_uint()?.expect_u32()?; let r = vr.expect_uint()?.expect_u32()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v % r) }
            Width::U64 => { let v = vl.expect_uint()?.expect_u64()?; let r = vr.expect_uint()?.expect_u64()?; if r == 0 { return Err("division by zero".into()) } Liternal::from(v % r) }
            Width::F32 => { let v = vl.expect_float()?.expect_f32()?; let r = vr.expect_float()?.expect_f32()?; if r == 0.0 { return Err("division by zero".into()) } Liternal::from(v % r) }
            Width::F64 => { let v = vl.expect_float()?.expect_f64()?; let r = vr.expect_float()?.expect_f64()?; if r == 0.0 { return Err("division by zero".into()) } Liternal::from(v % r) }
        };
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(result));
        Ok(())
    }

    pub(crate) fn neg(&mut self, dst: Addr, src: Addr, ty: Width) -> Result<(), String> {
        let val = self.read_one(src)?;
        let result = match ty {
            Width::I8 => Liternal::from(-val.expect_int()?.expect_i8()?),
            Width::I16 => Liternal::from(-val.expect_int()?.expect_i16()?),
            Width::I32 => Liternal::from(-val.expect_int()?.expect_i32()?),
            Width::I64 => Liternal::from(-val.expect_int()?.expect_i64()?),
            Width::F32 => Liternal::from(-val.expect_float()?.expect_f32()?),
            Width::F64 => Liternal::from(-val.expect_float()?.expect_f64()?),
            _ => return Err(format!("neg on unsigned type {:?}", ty)),
        };
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(result));
        Ok(())
    }
}
