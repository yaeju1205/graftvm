use graftvm_bytecode::{Addr, Width};

use crate::vm::VM;

impl VM {
    // ── arithmetic binops ──

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
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        match ty {
            Width::I8 => {
                let v = lhs_frag.data.expect_int()?.expect_i8()?;
                let r = rhs_frag.data.expect_int()?.expect_i8()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v / r));
            }
            Width::I16 => {
                let v = lhs_frag.data.expect_int()?.expect_i16()?;
                let r = rhs_frag.data.expect_int()?.expect_i16()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v / r));
            }
            Width::I32 => {
                let v = lhs_frag.data.expect_int()?.expect_i32()?;
                let r = rhs_frag.data.expect_int()?.expect_i32()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v / r));
            }
            Width::I64 => {
                let v = lhs_frag.data.expect_int()?.expect_i64()?;
                let r = rhs_frag.data.expect_int()?.expect_i64()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v / r));
            }
            Width::U8 => {
                let v = lhs_frag.data.expect_uint()?.expect_u8()?;
                let r = rhs_frag.data.expect_uint()?.expect_u8()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v / r));
            }
            Width::U16 => {
                let v = lhs_frag.data.expect_uint()?.expect_u16()?;
                let r = rhs_frag.data.expect_uint()?.expect_u16()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v / r));
            }
            Width::U32 => {
                let v = lhs_frag.data.expect_uint()?.expect_u32()?;
                let r = rhs_frag.data.expect_uint()?.expect_u32()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v / r));
            }
            Width::U64 => {
                let v = lhs_frag.data.expect_uint()?.expect_u64()?;
                let r = rhs_frag.data.expect_uint()?.expect_u64()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v / r));
            }
            Width::F32 => {
                let v = lhs_frag.data.expect_float()?.expect_f32()?;
                let r = rhs_frag.data.expect_float()?.expect_f32()?;
                if r == 0.0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v / r));
            }
            Width::F64 => {
                let v = lhs_frag.data.expect_float()?.expect_f64()?;
                let r = rhs_frag.data.expect_float()?.expect_f64()?;
                if r == 0.0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v / r));
            }
        }
        Ok(())
    }

    pub(crate) fn rem(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        match ty {
            Width::I8 => {
                let v = lhs_frag.data.expect_int()?.expect_i8()?;
                let r = rhs_frag.data.expect_int()?.expect_i8()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v % r));
            }
            Width::I16 => {
                let v = lhs_frag.data.expect_int()?.expect_i16()?;
                let r = rhs_frag.data.expect_int()?.expect_i16()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v % r));
            }
            Width::I32 => {
                let v = lhs_frag.data.expect_int()?.expect_i32()?;
                let r = rhs_frag.data.expect_int()?.expect_i32()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v % r));
            }
            Width::I64 => {
                let v = lhs_frag.data.expect_int()?.expect_i64()?;
                let r = rhs_frag.data.expect_int()?.expect_i64()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v % r));
            }
            Width::U8 => {
                let v = lhs_frag.data.expect_uint()?.expect_u8()?;
                let r = rhs_frag.data.expect_uint()?.expect_u8()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v % r));
            }
            Width::U16 => {
                let v = lhs_frag.data.expect_uint()?.expect_u16()?;
                let r = rhs_frag.data.expect_uint()?.expect_u16()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v % r));
            }
            Width::U32 => {
                let v = lhs_frag.data.expect_uint()?.expect_u32()?;
                let r = rhs_frag.data.expect_uint()?.expect_u32()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v % r));
            }
            Width::U64 => {
                let v = lhs_frag.data.expect_uint()?.expect_u64()?;
                let r = rhs_frag.data.expect_uint()?.expect_u64()?;
                if r == 0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v % r));
            }
            Width::F32 => {
                let v = lhs_frag.data.expect_float()?.expect_f32()?;
                let r = rhs_frag.data.expect_float()?.expect_f32()?;
                if r == 0.0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v % r));
            }
            Width::F64 => {
                let v = lhs_frag.data.expect_float()?.expect_f64()?;
                let r = rhs_frag.data.expect_float()?.expect_f64()?;
                if r == 0.0 {
                    return Err("division by zero".into());
                }
                *self.get_slot_mut(dst)? = Some(pipe_ws_lit!(v % r));
            }
        }
        Ok(())
    }

    pub(crate) fn neg(&mut self, dst: Addr, src: Addr, ty: Width) -> Result<(), String> {
        unop_width!(self, dst, src, ty, |v| -v)
    }
}
