use std::fmt::Debug;

use crate::data::Liternal;

#[derive(Clone, PartialEq)]
pub enum Kind {
    Int,
    UInt,
    Float,
    String,
    Bool,
}

impl Debug for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => f.write_str("int"),
            Self::UInt => f.write_str("uint"),
            Self::Float => f.write_str("float"),
            Self::String => f.write_str("string"),
            Self::Bool => f.write_str("bool"),
        }
    }
}

impl From<&Liternal> for Kind {
    fn from(data: &Liternal) -> Self {
        match data {
            Liternal::Int(_) => Self::Int,
            Liternal::UInt(_) => Self::UInt,
            Liternal::Float(_) => Self::Float,
            Liternal::String(_) => Self::String,
            Liternal::Bool(_) => Self::Bool,
        }
    }
}
