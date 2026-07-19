use graftvm_fragment::Fragment;

use crate::{opcode::Addr, vm::VM};

impl VM {
    pub(super) fn expect_number(&self, addr: Addr) -> Result<&Fragment, String> {
        if let Some(fragment) = self.get_slot(addr)? {
            Ok(fragment)
        } else {
            Err("expected number, found None".into())
        }
    }

    pub(super) fn expect_number_lhs_rhs(
        &self,
        lhs: Addr,
        rhs: Addr,
    ) -> Result<(&Fragment, &Fragment), String> {
        let lhs_fragment = self.expect_number(lhs)?;
        let rhs_fragment = self.expect_number(rhs)?;

        Ok((lhs_fragment, rhs_fragment))
    }
}
