use graftvm_bytecode::Addr;
use graftvm_liternal::{Float, Int, Liternal, UInt};
use graftvm_window::WindowSlot;

use crate::vm::VM;

impl VM {
    /// Sign-extend (signed=true) or zero-extend (signed=false) to the next wider integer type.
    pub(crate) fn extend(&mut self, dst: Addr, src: Addr, signed: bool) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = if signed {
            let i = f.data.expect_int()?;
            match i {
                Int::Int8(v) => Liternal::from(*v as i16),
                Int::Int16(v) => Liternal::from(*v as i32),
                Int::Int32(v) => Liternal::from(*v as i64),
                Int::Int64(_) => return Err("extend: i64 cannot be extended further".into()),
            }
        } else {
            let u = f.data.expect_uint()?;
            match u {
                UInt::UInt8(v) => Liternal::from(*v as u16),
                UInt::UInt16(v) => Liternal::from(*v as u32),
                UInt::UInt32(v) => Liternal::from(*v as u64),
                UInt::UInt64(_) => return Err("extend: u64 cannot be extended further".into()),
            }
        };
        *self.get_slot_mut(dst)? = Some(WindowSlot::from(v));
        Ok(())
    }

    /// Truncate to the next narrower integer type.
    pub(crate) fn trunc(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let i = f.data.expect_int()?;
        let v = match i {
            Int::Int64(v) => Liternal::from(*v as i32),
            Int::Int32(v) => Liternal::from(*v as i16),
            Int::Int16(v) => Liternal::from(*v as i8),
            Int::Int8(_) => return Err("trunc: i8 cannot be truncated further".into()),
        };
        *self.get_slot_mut(dst)? = Some(WindowSlot::from(v));
        Ok(())
    }

    /// Bit-preserving reinterpret between signed and unsigned of the same width.
    pub(crate) fn reinterpret(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = match &f.data {
            Liternal::Int(Int::Int8(v)) => Liternal::from(*v as u8),
            Liternal::Int(Int::Int16(v)) => Liternal::from(*v as u16),
            Liternal::Int(Int::Int32(v)) => Liternal::from(*v as u32),
            Liternal::Int(Int::Int64(v)) => Liternal::from(*v as u64),
            Liternal::UInt(UInt::UInt8(v)) => Liternal::from(*v as i8),
            Liternal::UInt(UInt::UInt16(v)) => Liternal::from(*v as i16),
            Liternal::UInt(UInt::UInt32(v)) => Liternal::from(*v as i32),
            Liternal::UInt(UInt::UInt64(v)) => Liternal::from(*v as i64),
            _ => return Err("reinterpret: source must be an integer type".into()),
        };
        *self.get_slot_mut(dst)? = Some(WindowSlot::from(v));
        Ok(())
    }

    /// Convert between integer and float types.
    /// signed=true: signed int ↔ float; signed=false: unsigned int ↔ float.
    pub(crate) fn convert(&mut self, dst: Addr, src: Addr, signed: bool) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = match (&f.data, signed) {
            // int → float
            (Liternal::Int(Int::Int64(v)), true) => Liternal::from(*v as f64),
            (Liternal::Int(Int::Int32(v)), true) => Liternal::from(*v as f32),
            (Liternal::Int(Int::Int16(v)), true) => Liternal::from(*v as f32),
            (Liternal::Int(Int::Int8(v)), true) => Liternal::from(*v as f32),
            // uint → float
            (Liternal::UInt(UInt::UInt64(v)), false) => Liternal::from(*v as f64),
            (Liternal::UInt(UInt::UInt32(v)), false) => Liternal::from(*v as f32),
            (Liternal::UInt(UInt::UInt16(v)), false) => Liternal::from(*v as f32),
            (Liternal::UInt(UInt::UInt8(v)), false) => Liternal::from(*v as f32),
            // float → int
            (Liternal::Float(Float::Float64(v)), true) => Liternal::from(*v as i64),
            (Liternal::Float(Float::Float32(v)), true) => Liternal::from(*v as i32),
            // float → uint
            (Liternal::Float(Float::Float64(v)), false) => Liternal::from(*v as u64),
            (Liternal::Float(Float::Float32(v)), false) => Liternal::from(*v as u32),
            _ => return Err("convert: unsupported type combination".into()),
        };
        *self.get_slot_mut(dst)? = Some(WindowSlot::from(v));
        Ok(())
    }
}
