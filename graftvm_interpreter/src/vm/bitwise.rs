use graftvm_bytecode::{Addr, Width};

use crate::vm::{binop_width, unop_width, VM};

impl VM {
    pub(crate) fn and(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        binop_width!(self, dst, lhs, rhs, ty, |v, r| v & r)
    }

    pub(crate) fn or(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        binop_width!(self, dst, lhs, rhs, ty, |v, r| v | r)
    }

    pub(crate) fn xor(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        binop_width!(self, dst, lhs, rhs, ty, |v, r| v ^ r)
    }

    pub(crate) fn not(&mut self, dst: Addr, src: Addr, ty: Width) -> Result<(), String> {
        unop_width!(self, dst, src, ty, |v| !v)
    }

    pub(crate) fn shl(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        binop_width!(self, dst, lhs, rhs, ty, |v, r| v << r)
    }

    pub(crate) fn shr(&mut self, dst: Addr, lhs: Addr, rhs: Addr, ty: Width) -> Result<(), String> {
        binop_width!(self, dst, lhs, rhs, ty, |v, r| v >> r)
    }
}
