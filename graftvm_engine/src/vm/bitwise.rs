use graftvm_fragment::Fragment;
use graftvm_liternal::Liternal;
use rpipe::pipe;

use crate::{opcode::Addr, vm::VM};

impl VM {
    pub(crate) fn andi8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data & rhs_data));
        Ok(())
    }

    pub(crate) fn andi16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data & rhs_data));
        Ok(())
    }

    pub(crate) fn andi32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data & rhs_data));
        Ok(())
    }

    pub(crate) fn andi64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data & rhs_data));
        Ok(())
    }

    pub(crate) fn ori8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data | rhs_data));
        Ok(())
    }

    pub(crate) fn ori16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data | rhs_data));
        Ok(())
    }

    pub(crate) fn ori32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data | rhs_data));
        Ok(())
    }

    pub(crate) fn ori64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data | rhs_data));
        Ok(())
    }

    pub(crate) fn xori8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data ^ rhs_data));
        Ok(())
    }

    pub(crate) fn xori16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data ^ rhs_data));
        Ok(())
    }

    pub(crate) fn xori32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data ^ rhs_data));
        Ok(())
    }

    pub(crate) fn xori64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data ^ rhs_data));
        Ok(())
    }

    pub(crate) fn noti8(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, !data));
        Ok(())
    }

    pub(crate) fn noti16(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, !data));
        Ok(())
    }

    pub(crate) fn noti32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, !data));
        Ok(())
    }

    pub(crate) fn noti64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, !data));
        Ok(())
    }

    pub(crate) fn shli8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data << rhs_data));
        Ok(())
    }

    pub(crate) fn shli16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data << rhs_data));
        Ok(())
    }

    pub(crate) fn shli32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data << rhs_data));
        Ok(())
    }

    pub(crate) fn shli64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data << rhs_data));
        Ok(())
    }

    pub(crate) fn shri8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i8()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data >> rhs_data));
        Ok(())
    }

    pub(crate) fn shri16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i16()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data >> rhs_data));
        Ok(())
    }

    pub(crate) fn shri32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i32()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data >> rhs_data));
        Ok(())
    }

    pub(crate) fn shri64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_int()?.expect_i64()?;
        let rhs_data = rhs_frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data >> rhs_data));
        Ok(())
    }

    pub(crate) fn andu8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data & rhs_data));
        Ok(())
    }

    pub(crate) fn andu16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data & rhs_data));
        Ok(())
    }

    pub(crate) fn andu32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data & rhs_data));
        Ok(())
    }

    pub(crate) fn andu64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data & rhs_data));
        Ok(())
    }

    pub(crate) fn oru8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data | rhs_data));
        Ok(())
    }

    pub(crate) fn oru16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data | rhs_data));
        Ok(())
    }

    pub(crate) fn oru32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data | rhs_data));
        Ok(())
    }

    pub(crate) fn oru64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data | rhs_data));
        Ok(())
    }

    pub(crate) fn xoru8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data ^ rhs_data));
        Ok(())
    }

    pub(crate) fn xoru16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data ^ rhs_data));
        Ok(())
    }

    pub(crate) fn xoru32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data ^ rhs_data));
        Ok(())
    }

    pub(crate) fn xoru64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, lhs_data ^ rhs_data));
        Ok(())
    }

    pub(crate) fn notu8(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, !data));
        Ok(())
    }

    pub(crate) fn notu16(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, !data));
        Ok(())
    }

    pub(crate) fn notu32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, !data));
        Ok(())
    }

    pub(crate) fn notu64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(Fragment::from, Liternal::from, !data));
        Ok(())
    }

    pub(crate) fn shlu8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data << rhs_data));
        Ok(())
    }

    pub(crate) fn shlu16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data << rhs_data));
        Ok(())
    }

    pub(crate) fn shlu32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data << rhs_data));
        Ok(())
    }

    pub(crate) fn shlu64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data << rhs_data));
        Ok(())
    }

    pub(crate) fn shru8(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u8()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data >> rhs_data));
        Ok(())
    }

    pub(crate) fn shru16(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u16()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data >> rhs_data));
        Ok(())
    }

    pub(crate) fn shru32(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u32()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data >> rhs_data));
        Ok(())
    }

    pub(crate) fn shru64(&mut self, dst: Addr, lhs: Addr, rhs: Addr) -> Result<(), String> {
        let (lhs_frag, rhs_frag) = self.expect_number_lhs_rhs(lhs, rhs)?;
        let lhs_data = lhs_frag.data.expect_uint()?.expect_u64()?;
        let rhs_data = rhs_frag.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? =
            Some(pipe!(Fragment::from, Liternal::from, lhs_data >> rhs_data));
        Ok(())
    }
}
