use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub enum Int {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
}

impl Int {
    pub fn as_i8(&self) -> Option<i8> {
        match self {
            Int::Int8(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_i16(&self) -> Option<i16> {
        match self {
            Int::Int16(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Int::Int32(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Int::Int64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn expect_i8(&self) -> Result<i8, String> {
        self.as_i8()
            .ok_or_else(|| format!("expected Int8, got {:?}", self))
    }
    pub fn expect_i16(&self) -> Result<i16, String> {
        self.as_i16()
            .ok_or_else(|| format!("expected Int16, got {:?}", self))
    }
    pub fn expect_i32(&self) -> Result<i32, String> {
        self.as_i32()
            .ok_or_else(|| format!("expected Int32, got {:?}", self))
    }
    pub fn expect_i64(&self) -> Result<i64, String> {
        self.as_i64()
            .ok_or_else(|| format!("expected Int64, got {:?}", self))
    }
}

impl Debug for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int8(_) => f.write_str("Int8"),
            Self::Int16(_) => f.write_str("Int16"),
            Self::Int32(_) => f.write_str("Int32"),
            Self::Int64(_) => f.write_str("Int64"),
        }
    }
}

impl From<i8> for Int {
    fn from(v: i8) -> Self {
        Int::Int8(v)
    }
}
impl From<i16> for Int {
    fn from(v: i16) -> Self {
        Int::Int16(v)
    }
}
impl From<i32> for Int {
    fn from(v: i32) -> Self {
        Int::Int32(v)
    }
}
impl From<i64> for Int {
    fn from(v: i64) -> Self {
        Int::Int64(v)
    }
}
