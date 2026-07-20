use graftvm_bytecode::Addr;
use graftvm_liternal::Liternal;
use graftvm_window::WindowSlot;
use rpipe::pipe;

use crate::vm::VM;

impl VM {
    pub(crate) fn negi8(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? = Some(pipe!(WindowSlot::from, Liternal::from, -data));
        Ok(())
    }

    pub(crate) fn negi16(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? = Some(pipe!(WindowSlot::from, Liternal::from, -data));
        Ok(())
    }

    pub(crate) fn negi32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(WindowSlot::from, Liternal::from, -data));
        Ok(())
    }

    pub(crate) fn negi64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(WindowSlot::from, Liternal::from, -data));
        Ok(())
    }

    pub(crate) fn negf32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_float()?.expect_f32()?;
        *self.get_slot_mut(dst)? = Some(pipe!(WindowSlot::from, Liternal::from, -data));
        Ok(())
    }

    pub(crate) fn negf64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let frag = self.expect_number(src)?;
        let data = frag.data.expect_float()?.expect_f64()?;
        *self.get_slot_mut(dst)? = Some(pipe!(WindowSlot::from, Liternal::from, -data));
        Ok(())
    }
}
