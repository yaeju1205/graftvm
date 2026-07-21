use graftvm_bytecode::Addr;
use graftvm_liternal::{Float, Int, Liternal, UInt};
use graftvm_window::WindowSlot;

use crate::vm::VM;

impl VM {
    pub(crate) fn extend(&mut self, dst: Addr, src: Addr, signed: bool) -> Result<(), String> {
        let val = self.read_one(src)?;
        let v = if signed {
            match val.expect_int()? {
                Int::Int8(v) => Liternal::from(*v as i16),
                Int::Int16(v) => Liternal::from(*v as i32),
                Int::Int32(v) => Liternal::from(*v as i64),
                Int::Int64(_) => return Err("cannot extend i64".into()),
            }
        } else {
            match val.expect_uint()? {
                UInt::UInt8(v) => Liternal::from(*v as u16),
                UInt::UInt16(v) => Liternal::from(*v as u32),
                UInt::UInt32(v) => Liternal::from(*v as u64),
                UInt::UInt64(_) => return Err("cannot extend u64".into()),
            }
        };
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(v));
        Ok(())
    }

    pub(crate) fn trunc(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let val = self.read_one(src)?;
        let v = match val.expect_int()? {
            Int::Int64(v) => Liternal::from(*v as i32),
            Int::Int32(v) => Liternal::from(*v as i16),
            Int::Int16(v) => Liternal::from(*v as i8),
            Int::Int8(_) => return Err("cannot trunc i8".into()),
        };
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(v));
        Ok(())
    }

    pub(crate) fn reinterpret(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let val = self.read_one(src)?;
        let v = match &val {
            Liternal::Int(Int::Int8(v)) => Liternal::from(*v as u8),
            Liternal::Int(Int::Int16(v)) => Liternal::from(*v as u16),
            Liternal::Int(Int::Int32(v)) => Liternal::from(*v as u32),
            Liternal::Int(Int::Int64(v)) => Liternal::from(*v as u64),
            Liternal::UInt(UInt::UInt8(v)) => Liternal::from(*v as i8),
            Liternal::UInt(UInt::UInt16(v)) => Liternal::from(*v as i16),
            Liternal::UInt(UInt::UInt32(v)) => Liternal::from(*v as i32),
            Liternal::UInt(UInt::UInt64(v)) => Liternal::from(*v as i64),
            _ => return Err("reinterpret: expected integer type".into()),
        };
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(v));
        Ok(())
    }

    pub(crate) fn convert(&mut self, dst: Addr, src: Addr, signed: bool) -> Result<(), String> {
        let val = self.read_one(src)?;
        let v = match (&val, signed) {
            (Liternal::Int(Int::Int64(v)), true) => Liternal::from(*v as f64),
            (Liternal::Int(Int::Int32(v)), true) => Liternal::from(*v as f32),
            (Liternal::Int(Int::Int16(v)), true) => Liternal::from(*v as f32),
            (Liternal::Int(Int::Int8(v)), true) => Liternal::from(*v as f32),
            (Liternal::UInt(UInt::UInt64(v)), false) => Liternal::from(*v as f64),
            (Liternal::UInt(UInt::UInt32(v)), false) => Liternal::from(*v as f32),
            (Liternal::UInt(UInt::UInt16(v)), false) => Liternal::from(*v as f32),
            (Liternal::UInt(UInt::UInt8(v)), false) => Liternal::from(*v as f32),
            (Liternal::Float(Float::Float64(v)), true) => Liternal::from(*v as i64),
            (Liternal::Float(Float::Float32(v)), true) => Liternal::from(*v as i32),
            (Liternal::Float(Float::Float64(v)), false) => Liternal::from(*v as u64),
            (Liternal::Float(Float::Float32(v)), false) => Liternal::from(*v as u32),
            _ => return Err("convert: unsupported type combination".into()),
        };
        *self.slot_mut(dst.slot) = Some(WindowSlot::from(v));
        Ok(())
    }
}
