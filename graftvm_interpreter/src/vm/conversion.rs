use graftvm_bytecode::Addr;
use graftvm_liternal::Liternal;
use graftvm_window::WindowSlot;
use rpipe::pipe;

use crate::vm::VM;

impl VM {
    pub(crate) fn extendi8_i16(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i16);
        Ok(())
    }

    pub(crate) fn extendi8_i32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i32);
        Ok(())
    }

    pub(crate) fn extendi8_i64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i64);
        Ok(())
    }

    pub(crate) fn extendi16_i32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i32);
        Ok(())
    }

    pub(crate) fn extendi16_i64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i64);
        Ok(())
    }

    pub(crate) fn extendi32_i64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i64);
        Ok(())
    }

    pub(crate) fn extendu8_u16(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as u16);
        Ok(())
    }

    pub(crate) fn extendu8_u32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as u32);
        Ok(())
    }

    pub(crate) fn extendu8_u64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as u64);
        Ok(())
    }

    pub(crate) fn extendu16_u32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as u32);
        Ok(())
    }

    pub(crate) fn extendu16_u64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as u64);
        Ok(())
    }

    pub(crate) fn extendu32_u64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as u64);
        Ok(())
    }

    pub(crate) fn trunci64_i32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i32);
        Ok(())
    }

    pub(crate) fn trunci64_i16(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i16);
        Ok(())
    }

    pub(crate) fn trunci64_i8(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i8);
        Ok(())
    }

    pub(crate) fn trunci32_i16(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i16);
        Ok(())
    }

    pub(crate) fn trunci32_i8(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i8);
        Ok(())
    }

    pub(crate) fn trunci16_i8(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i8);
        Ok(())
    }

    // ── reinterpret (same-width, bit-preserving) ──

    pub(crate) fn reinterpu8_i8(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_uint()?.expect_u8()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i8);
        Ok(())
    }

    pub(crate) fn reinterpu16_i16(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_uint()?.expect_u16()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i16);
        Ok(())
    }

    pub(crate) fn reinterpu32_i32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_uint()?.expect_u32()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i32);
        Ok(())
    }

    pub(crate) fn reinterpu64_i64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i64);
        Ok(())
    }

    pub(crate) fn reinterpi8_u8(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i8()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as u8);
        Ok(())
    }

    pub(crate) fn reinterpi16_u16(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i16()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as u16);
        Ok(())
    }

    pub(crate) fn reinterpi32_u32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i32()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as u32);
        Ok(())
    }

    pub(crate) fn reinterpi64_u64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as u64);
        Ok(())
    }

    pub(crate) fn inttof32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as f32);
        Ok(())
    }

    pub(crate) fn inttof64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_int()?.expect_i64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as f64);
        Ok(())
    }

    pub(crate) fn uinttof32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as f32);
        Ok(())
    }

    pub(crate) fn uinttof64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_uint()?.expect_u64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as f64);
        Ok(())
    }

    pub(crate) fn ftoint32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_float()?.expect_f64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i32);
        Ok(())
    }

    pub(crate) fn ftoint64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_float()?.expect_f64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as i64);
        Ok(())
    }

    pub(crate) fn ftouint32(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_float()?.expect_f64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as u32);
        Ok(())
    }

    pub(crate) fn ftouint64(&mut self, dst: Addr, src: Addr) -> Result<(), String> {
        let f = self.expect_number(src)?;
        let v = f.data.expect_float()?.expect_f64()?;
        *self.get_slot_mut(dst)? = pipe!(Some, WindowSlot::from, Liternal::from, v as u64);
        Ok(())
    }
}
