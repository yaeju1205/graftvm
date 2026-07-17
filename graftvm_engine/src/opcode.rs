use graftvm_data::data::Liternal;

#[derive(Clone)]
pub struct Addr {
    pub window: usize,
    pub slot: usize,
}

#[derive(Clone)]
pub enum Opcode {
    Add { dst: Addr, lhs: Addr, rhs: Addr },
    Sub { dst: Addr, lhs: Addr, rhs: Addr },
    Mul { dst: Addr, lhs: Addr, rhs: Addr },
    Div { dst: Addr, lhs: Addr, rhs: Addr },

    Lt { dst: Addr, lhs: Addr, rhs: Addr },
    Gt { dst: Addr, lhs: Addr, rhs: Addr },
    Lte { dst: Addr, lhs: Addr, rhs: Addr },
    Gte { dst: Addr, lhs: Addr, rhs: Addr },
    Eq { dst: Addr, lhs: Addr, rhs: Addr },

    Jump(usize),
    Branch(usize),

    StoreData { index: usize, data: Liternal },
    LoadData { dst: Addr, index: usize },

    Move { dst: Addr, src: Addr },
    Copy { dst: Addr, src: Addr },
    Drop { src: Addr },

    Call { dst: Addr, index: usize, arg: Addr },

    Enter,
    Exit,
}
