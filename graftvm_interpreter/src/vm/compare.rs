use graftvm_bytecode::Addr;
use graftvm_liternal::Liternal;
use graftvm_window::WindowSlot;
use rpipe::pipe;

use crate::vm::VM;

impl VM {
    pub(crate) fn lti8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data < rhs_data));
        Ok(())
    }

    pub(crate) fn lti16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data < rhs_data));
        Ok(())
    }

    pub(crate) fn lti32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data < rhs_data));
        Ok(())
    }

    pub(crate) fn lti64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data < rhs_data));
        Ok(())
    }

    pub(crate) fn lei8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data <= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn lei16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data <= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn lei32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data <= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn lei64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data <= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn gti8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data > rhs_data));
        Ok(())
    }

    pub(crate) fn gti16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data > rhs_data));
        Ok(())
    }

    pub(crate) fn gti32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data > rhs_data));
        Ok(())
    }

    pub(crate) fn gti64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data > rhs_data));
        Ok(())
    }

    pub(crate) fn gei8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data >= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn gei16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data >= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn gei32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data >= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn gei64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data >= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn ltu8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data < rhs_data));
        Ok(())
    }

    pub(crate) fn ltu16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data < rhs_data));
        Ok(())
    }

    pub(crate) fn ltu32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data < rhs_data));
        Ok(())
    }

    pub(crate) fn ltu64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data < rhs_data));
        Ok(())
    }

    pub(crate) fn leu8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data <= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn leu16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data <= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn leu32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data <= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn leu64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data <= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn gtu8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data > rhs_data));
        Ok(())
    }

    pub(crate) fn gtu16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data > rhs_data));
        Ok(())
    }

    pub(crate) fn gtu32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data > rhs_data));
        Ok(())
    }

    pub(crate) fn gtu64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data > rhs_data));
        Ok(())
    }

    pub(crate) fn geu8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data >= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn geu16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data >= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn geu32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data >= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn geu64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data >= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn ltf32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f32()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data < rhs_data));
        Ok(())
    }

    pub(crate) fn ltf64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f64()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data < rhs_data));
        Ok(())
    }

    pub(crate) fn lef32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f32()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data <= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn lef64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f64()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data <= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn gtf32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f32()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data > rhs_data));
        Ok(())
    }

    pub(crate) fn gtf64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f64()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(WindowSlot::from, Liternal::from, lhs_data > rhs_data));
        Ok(())
    }

    pub(crate) fn gef32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f32()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data >= rhs_data
        ));
        Ok(())
    }

    pub(crate) fn gef64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_float()?.expect_f64()?;
        let rhs_data = rhs_frag.data.expect_float()?.expect_f64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(
            WindowSlot::from,
            Liternal::from,
            lhs_data >= rhs_data
        ));
        Ok(())
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
