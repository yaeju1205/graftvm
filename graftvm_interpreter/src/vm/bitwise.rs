use graftvm_bytecode::{Addr, Width};
use graftvm_liternal::Liternal;
use graftvm_window::WindowSlot;

use crate::vm::VM;

/// Bitwise ops only apply to integer types (signed and unsigned), not floats.
fn bit_result(vl: Liternal, vr: Liternal, ty: Width) -> Result<Liternal, String> {
    Ok(match ty {
        Width::I8 => Liternal::from(vl.expect_int()?.expect_i8()? & vr.expect_int()?.expect_i8()?),
        Width::I16 => Liternal::from(vl.expect_int()?.expect_i16()? & vr.expect_int()?.expect_i16()?),
        Width::I32 => Liternal::from(vl.expect_int()?.expect_i32()? & vr.expect_int()?.expect_i32()?),
        Width::I64 => Liternal::from(vl.expect_int()?.expect_i64()? & vr.expect_int()?.expect_i64()?),
        Width::U8 => Liternal::from(vl.expect_uint()?.expect_u8()? & vr.expect_uint()?.expect_u8()?),
        Width::U16 => Liternal::from(vl.expect_uint()?.expect_u16()? & vr.expect_uint()?.expect_u16()?),
        Width::U32 => Liternal::from(vl.expect_uint()?.expect_u32()? & vr.expect_uint()?.expect_u32()?),
        Width::U64 => Liternal::from(vl.expect_uint()?.expect_u64()? & vr.expect_uint()?.expect_u64()?),
        _ => return Err(format!("bitwise not supported on {:?}", ty)),
    })
}

fn bor_result(vl: Liternal, vr: Liternal, ty: Width) -> Result<Liternal, String> {
    Ok(match ty {
        Width::I8 => Liternal::from(vl.expect_int()?.expect_i8()? | vr.expect_int()?.expect_i8()?),
        Width::I16 => Liternal::from(vl.expect_int()?.expect_i16()? | vr.expect_int()?.expect_i16()?),
        Width::I32 => Liternal::from(vl.expect_int()?.expect_i32()? | vr.expect_int()?.expect_i32()?),
        Width::I64 => Liternal::from(vl.expect_int()?.expect_i64()? | vr.expect_int()?.expect_i64()?),
        Width::U8 => Liternal::from(vl.expect_uint()?.expect_u8()? | vr.expect_uint()?.expect_u8()?),
        Width::U16 => Liternal::from(vl.expect_uint()?.expect_u16()? | vr.expect_uint()?.expect_u16()?),
        Width::U32 => Liternal::from(vl.expect_uint()?.expect_u32()? | vr.expect_uint()?.expect_u32()?),
        Width::U64 => Liternal::from(vl.expect_uint()?.expect_u64()? | vr.expect_uint()?.expect_u64()?),
        _ => return Err(format!("bitwise not supported on {:?}", ty)),
    })
}

fn bxor_result(vl: Liternal, vr: Liternal, ty: Width) -> Result<Liternal, String> {
    Ok(match ty {
        Width::I8 => Liternal::from(vl.expect_int()?.expect_i8()? ^ vr.expect_int()?.expect_i8()?),
        Width::I16 => Liternal::from(vl.expect_int()?.expect_i16()? ^ vr.expect_int()?.expect_i16()?),
        Width::I32 => Liternal::from(vl.expect_int()?.expect_i32()? ^ vr.expect_int()?.expect_i32()?),
        Width::I64 => Liternal::from(vl.expect_int()?.expect_i64()? ^ vr.expect_int()?.expect_i64()?),
        Width::U8 => Liternal::from(vl.expect_uint()?.expect_u8()? ^ vr.expect_uint()?.expect_u8()?),
        Width::U16 => Liternal::from(vl.expect_uint()?.expect_u16()? ^ vr.expect_uint()?.expect_u16()?),
        Width::U32 => Liternal::from(vl.expect_uint()?.expect_u32()? ^ vr.expect_uint()?.expect_u32()?),
        Width::U64 => Liternal::from(vl.expect_uint()?.expect_u64()? ^ vr.expect_uint()?.expect_u64()?),
        _ => return Err(format!("bitwise not supported on {:?}", ty)),
    })
}

fn not_result(val: Liternal, ty: Width) -> Result<Liternal, String> {
    Ok(match ty {
        Width::I8 => Liternal::from(!val.expect_int()?.expect_i8()?),
        Width::I16 => Liternal::from(!val.expect_int()?.expect_i16()?),
        Width::I32 => Liternal::from(!val.expect_int()?.expect_i32()?),
        Width::I64 => Liternal::from(!val.expect_int()?.expect_i64()?),
        Width::U8 => Liternal::from(!val.expect_uint()?.expect_u8()?),
        Width::U16 => Liternal::from(!val.expect_uint()?.expect_u16()?),
        Width::U32 => Liternal::from(!val.expect_uint()?.expect_u32()?),
        Width::U64 => Liternal::from(!val.expect_uint()?.expect_u64()?),
        _ => return Err(format!("bitwise not supported on {:?}", ty)),
    })
}

fn shift_result(vl: Liternal, vr: Liternal, ty: Width) -> Result<Liternal, String> {
    Ok(match ty {
        Width::I8 => Liternal::from(vl.expect_int()?.expect_i8()? << vr.expect_int()?.expect_i8()?),
        Width::I16 => Liternal::from(vl.expect_int()?.expect_i16()? << vr.expect_int()?.expect_i16()?),
        Width::I32 => Liternal::from(vl.expect_int()?.expect_i32()? << vr.expect_int()?.expect_i32()?),
        Width::I64 => Liternal::from(vl.expect_int()?.expect_i64()? << vr.expect_int()?.expect_i64()?),
        Width::U8 => Liternal::from(vl.expect_uint()?.expect_u8()? << vr.expect_uint()?.expect_u8()?),
        Width::U16 => Liternal::from(vl.expect_uint()?.expect_u16()? << vr.expect_uint()?.expect_u16()?),
        Width::U32 => Liternal::from(vl.expect_uint()?.expect_u32()? << vr.expect_uint()?.expect_u32()?),
        Width::U64 => Liternal::from(vl.expect_uint()?.expect_u64()? << vr.expect_uint()?.expect_u64()?),
        _ => return Err(format!("shift not supported on {:?}", ty)),
    })
}

impl VM {
    pub(crate) fn and(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        let result = bit_result(vl, vr, ty)?;
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(result));
        Ok(())
    }

    pub(crate) fn or(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        let result = bor_result(vl, vr, ty)?;
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(result));
        Ok(())
    }

    pub(crate) fn xor(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        let result = bxor_result(vl, vr, ty)?;
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(result));
        Ok(())
    }

    pub(crate) fn not(&mut self, dst: Addr, src: Addr, ty: Width) -> Result<(), String> {
        let val = self.read_one(src)?;
        let result = not_result(val, ty)?;
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(result));
        Ok(())
    }

    pub(crate) fn shl(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        let result = shift_result(vl, vr, ty)?;
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(result));
        Ok(())
    }

    pub(crate) fn shr(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        let (vl, vr) = self.read_two(lhs, rhs)?;
        let result = shift_result(vl, vr, ty)?;
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(result));
        Ok(())
    }
}
