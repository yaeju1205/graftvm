use graftvm_bytecode::Addr;
use graftvm_liternal::Liternal;
use graftvm_window::WindowSlot;
use rpipe::pipe;

use crate::vm::VM;

impl VM {
    pub(crate) fn addi8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data + rhs_data));
        Ok(())
    }

    pub(crate) fn addi16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data + rhs_data));
        Ok(())
    }

    pub(crate) fn addi32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data + rhs_data));
        Ok(())
    }

    pub(crate) fn addi64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data + rhs_data));
        Ok(())
    }

    pub(crate) fn subi8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data - rhs_data));
        Ok(())
    }

    pub(crate) fn subi16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data - rhs_data));
        Ok(())
    }

    pub(crate) fn subi32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data - rhs_data));
        Ok(())
    }

    pub(crate) fn subi64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data - rhs_data));
        Ok(())
    }

    pub(crate) fn muli8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data * rhs_data));
        Ok(())
    }

    pub(crate) fn muli16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data * rhs_data));
        Ok(())
    }

    pub(crate) fn muli32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data * rhs_data));
        Ok(())
    }

    pub(crate) fn muli64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data * rhs_data));
        Ok(())
    }

    pub(crate) fn divi8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data / rhs_data));
        Ok(())
    }

    pub(crate) fn divi16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data / rhs_data));
        Ok(())
    }

    pub(crate) fn divi32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data / rhs_data));
        Ok(())
    }

    pub(crate) fn divi64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data / rhs_data));
        Ok(())
    }

    pub(crate) fn remi8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data % rhs_data));
        Ok(())
    }

    pub(crate) fn remi16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data % rhs_data));
        Ok(())
    }

    pub(crate) fn remi32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data % rhs_data));
        Ok(())
    }

    pub(crate) fn remi64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data % rhs_data));
        Ok(())
    }

    pub(crate) fn addu8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data + rhs_data));
        Ok(())
    }

    pub(crate) fn addu16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data + rhs_data));
        Ok(())
    }

    pub(crate) fn addu32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data + rhs_data));
        Ok(())
    }

    pub(crate) fn addu64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data + rhs_data));
        Ok(())
    }

    pub(crate) fn subu8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data - rhs_data));
        Ok(())
    }

    pub(crate) fn subu16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data - rhs_data));
        Ok(())
    }

    pub(crate) fn subu32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data - rhs_data));
        Ok(())
    }

    pub(crate) fn subu64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data - rhs_data));
        Ok(())
    }

    pub(crate) fn mulu8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data * rhs_data));
        Ok(())
    }

    pub(crate) fn mulu16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data * rhs_data));
        Ok(())
    }

    pub(crate) fn mulu32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data * rhs_data));
        Ok(())
    }

    pub(crate) fn mulu64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data * rhs_data));
        Ok(())
    }

    pub(crate) fn divu8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data / rhs_data));
        Ok(())
    }

    pub(crate) fn divu16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data / rhs_data));
        Ok(())
    }

    pub(crate) fn divu32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data / rhs_data));
        Ok(())
    }

    pub(crate) fn divu64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data / rhs_data));
        Ok(())
    }

    pub(crate) fn remu8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data % rhs_data));
        Ok(())
    }

    pub(crate) fn remu16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data % rhs_data));
        Ok(())
    }

    pub(crate) fn remu32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data % rhs_data));
        Ok(())
    }

    pub(crate) fn remu64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        if rhs_data == 0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data % rhs_data));
        Ok(())
    }

    pub(crate) fn addf32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f32()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data + rhs_data));
        Ok(())
    }

    pub(crate) fn addf64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f64()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data + rhs_data));
        Ok(())
    }

    pub(crate) fn subf32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f32()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data - rhs_data));
        Ok(())
    }

    pub(crate) fn subf64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f64()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data - rhs_data));
        Ok(())
    }

    pub(crate) fn mulf32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f32()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data * rhs_data));
        Ok(())
    }

    pub(crate) fn mulf64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f64()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data * rhs_data));
        Ok(())
    }

    pub(crate) fn divf32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f32()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f32()?;
        if rhs_data == 0. {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data / rhs_data));
        Ok(())
    }

    pub(crate) fn divf64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f64()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f64()?;
        if rhs_data == 0.0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data / rhs_data));
        Ok(())
    }

    pub(crate) fn remf32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f32()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f32()?;
        if rhs_data == 0.0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data % rhs_data));
        Ok(())
    }

    pub(crate) fn remf64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f64()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f64()?;
        if rhs_data == 0.0 {
            return Err("division by zero".into());
        }
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data % rhs_data));
        Ok(())
    }
}
