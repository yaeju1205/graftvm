use graftvm_bytecode::{Addr, Width};
use graftvm_liternal::Liternal;
use graftvm_window::WindowSlot;

use crate::vm::VM;

macro_rules! cmp_op {
    ($self:expr, $dst:expr, $lhs:expr, $rhs:expr, $ty:expr, lt) => {{
        let (lhs_frag, rhs_frag) = $self.expect_number_lhs_rhs($lhs, $rhs)?;
        let result = match $ty {
            Width::I8 => (lhs_frag.data.expect_int()?.expect_i8()? as i64) < (rhs_frag.data.expect_int()?.expect_i8()? as i64),
            Width::I16 => (lhs_frag.data.expect_int()?.expect_i16()? as i64) < (rhs_frag.data.expect_int()?.expect_i16()? as i64),
            Width::I32 => (lhs_frag.data.expect_int()?.expect_i32()? as i64) < (rhs_frag.data.expect_int()?.expect_i32()? as i64),
            Width::I64 => lhs_frag.data.expect_int()?.expect_i64()? < rhs_frag.data.expect_int()?.expect_i64()?,
            Width::U8 => (lhs_frag.data.expect_uint()?.expect_u8()? as u64) < (rhs_frag.data.expect_uint()?.expect_u8()? as u64),
            Width::U16 => (lhs_frag.data.expect_uint()?.expect_u16()? as u64) < (rhs_frag.data.expect_uint()?.expect_u16()? as u64),
            Width::U32 => (lhs_frag.data.expect_uint()?.expect_u32()? as u64) < (rhs_frag.data.expect_uint()?.expect_u32()? as u64),
            Width::U64 => lhs_frag.data.expect_uint()?.expect_u64()? < rhs_frag.data.expect_uint()?.expect_u64()?,
            Width::F32 => (lhs_frag.data.expect_float()?.expect_f32()? as f64) < (rhs_frag.data.expect_float()?.expect_f32()? as f64),
            Width::F64 => lhs_frag.data.expect_float()?.expect_f64()? < rhs_frag.data.expect_float()?.expect_f64()?,
        };
        *$self.get_slot_mut($dst)? = Some(WindowSlot::from(Liternal::from(result)));
        Ok(())
    }};
    ($self:expr, $dst:expr, $lhs:expr, $rhs:expr, $ty:expr, le) => {{
        let (lhs_frag, rhs_frag) = $self.expect_number_lhs_rhs($lhs, $rhs)?;
        let result = match $ty {
            Width::I8 => (lhs_frag.data.expect_int()?.expect_i8()? as i64) <= (rhs_frag.data.expect_int()?.expect_i8()? as i64),
            Width::I16 => (lhs_frag.data.expect_int()?.expect_i16()? as i64) <= (rhs_frag.data.expect_int()?.expect_i16()? as i64),
            Width::I32 => (lhs_frag.data.expect_int()?.expect_i32()? as i64) <= (rhs_frag.data.expect_int()?.expect_i32()? as i64),
            Width::I64 => lhs_frag.data.expect_int()?.expect_i64()? <= rhs_frag.data.expect_int()?.expect_i64()?,
            Width::U8 => (lhs_frag.data.expect_uint()?.expect_u8()? as u64) <= (rhs_frag.data.expect_uint()?.expect_u8()? as u64),
            Width::U16 => (lhs_frag.data.expect_uint()?.expect_u16()? as u64) <= (rhs_frag.data.expect_uint()?.expect_u16()? as u64),
            Width::U32 => (lhs_frag.data.expect_uint()?.expect_u32()? as u64) <= (rhs_frag.data.expect_uint()?.expect_u32()? as u64),
            Width::U64 => lhs_frag.data.expect_uint()?.expect_u64()? <= rhs_frag.data.expect_uint()?.expect_u64()?,
            Width::F32 => (lhs_frag.data.expect_float()?.expect_f32()? as f64) <= (rhs_frag.data.expect_float()?.expect_f32()? as f64),
            Width::F64 => lhs_frag.data.expect_float()?.expect_f64()? <= rhs_frag.data.expect_float()?.expect_f64()?,
        };
        *$self.get_slot_mut($dst)? = Some(WindowSlot::from(Liternal::from(result)));
        Ok(())
    }};
    ($self:expr, $dst:expr, $lhs:expr, $rhs:expr, $ty:expr, gt) => {{
        let (lhs_frag, rhs_frag) = $self.expect_number_lhs_rhs($lhs, $rhs)?;
        let result = match $ty {
            Width::I8 => (lhs_frag.data.expect_int()?.expect_i8()? as i64) > (rhs_frag.data.expect_int()?.expect_i8()? as i64),
            Width::I16 => (lhs_frag.data.expect_int()?.expect_i16()? as i64) > (rhs_frag.data.expect_int()?.expect_i16()? as i64),
            Width::I32 => (lhs_frag.data.expect_int()?.expect_i32()? as i64) > (rhs_frag.data.expect_int()?.expect_i32()? as i64),
            Width::I64 => lhs_frag.data.expect_int()?.expect_i64()? > rhs_frag.data.expect_int()?.expect_i64()?,
            Width::U8 => (lhs_frag.data.expect_uint()?.expect_u8()? as u64) > (rhs_frag.data.expect_uint()?.expect_u8()? as u64),
            Width::U16 => (lhs_frag.data.expect_uint()?.expect_u16()? as u64) > (rhs_frag.data.expect_uint()?.expect_u16()? as u64),
            Width::U32 => (lhs_frag.data.expect_uint()?.expect_u32()? as u64) > (rhs_frag.data.expect_uint()?.expect_u32()? as u64),
            Width::U64 => lhs_frag.data.expect_uint()?.expect_u64()? > rhs_frag.data.expect_uint()?.expect_u64()?,
            Width::F32 => (lhs_frag.data.expect_float()?.expect_f32()? as f64) > (rhs_frag.data.expect_float()?.expect_f32()? as f64),
            Width::F64 => lhs_frag.data.expect_float()?.expect_f64()? > rhs_frag.data.expect_float()?.expect_f64()?,
        };
        *$self.get_slot_mut($dst)? = Some(WindowSlot::from(Liternal::from(result)));
        Ok(())
    }};
    ($self:expr, $dst:expr, $lhs:expr, $rhs:expr, $ty:expr, ge) => {{
        let (lhs_frag, rhs_frag) = $self.expect_number_lhs_rhs($lhs, $rhs)?;
        let result = match $ty {
            Width::I8 => (lhs_frag.data.expect_int()?.expect_i8()? as i64) >= (rhs_frag.data.expect_int()?.expect_i8()? as i64),
            Width::I16 => (lhs_frag.data.expect_int()?.expect_i16()? as i64) >= (rhs_frag.data.expect_int()?.expect_i16()? as i64),
            Width::I32 => (lhs_frag.data.expect_int()?.expect_i32()? as i64) >= (rhs_frag.data.expect_int()?.expect_i32()? as i64),
            Width::I64 => lhs_frag.data.expect_int()?.expect_i64()? >= rhs_frag.data.expect_int()?.expect_i64()?,
            Width::U8 => (lhs_frag.data.expect_uint()?.expect_u8()? as u64) >= (rhs_frag.data.expect_uint()?.expect_u8()? as u64),
            Width::U16 => (lhs_frag.data.expect_uint()?.expect_u16()? as u64) >= (rhs_frag.data.expect_uint()?.expect_u16()? as u64),
            Width::U32 => (lhs_frag.data.expect_uint()?.expect_u32()? as u64) >= (rhs_frag.data.expect_uint()?.expect_u32()? as u64),
            Width::U64 => lhs_frag.data.expect_uint()?.expect_u64()? >= rhs_frag.data.expect_uint()?.expect_u64()?,
            Width::F32 => (lhs_frag.data.expect_float()?.expect_f32()? as f64) >= (rhs_frag.data.expect_float()?.expect_f32()? as f64),
            Width::F64 => lhs_frag.data.expect_float()?.expect_f64()? >= rhs_frag.data.expect_float()?.expect_f64()?,
        };
        *$self.get_slot_mut($dst)? = Some(WindowSlot::from(Liternal::from(result)));
        Ok(())
    }};
}

impl VM {
    pub(crate) fn lt(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        cmp_op!(self, dst, lhs, rhs, ty, lt)
    }

    pub(crate) fn le(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        cmp_op!(self, dst, lhs, rhs, ty, le)
    }

    pub(crate) fn gt(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        cmp_op!(self, dst, lhs, rhs, ty, gt)
    }

    pub(crate) fn ge(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        cmp_op!(self, dst, lhs, rhs, ty, ge)
    }

    pub(crate) fn eq(&mut self, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let lhs_slot = self.get_slot(lhs)?;
        let rhs_slot = self.get_slot(rhs)?;
        self.state.cmp = match lhs_slot {
            Some(lhs_frag) => match rhs_slot {
                Some(rhs_frag) => lhs_frag.data == rhs_frag.data,
                None => false,
            },
            None => rhs_slot.is_none(),
        };
        Ok(())
    }

    pub(crate) fn neq(&mut self, lhs: Addr, rhs: Addr) -> Result<(), String> {
        self.eq(lhs, rhs)?;
        self.state.cmp = !self.state.cmp;
        Ok(())
    }
}
