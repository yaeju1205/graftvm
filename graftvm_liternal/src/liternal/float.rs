use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub enum Float {
    Float32(f32),
    Float64(f64),
}

impl Float {
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Float::Float32(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Float::Float64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn expect_f32(&self) -> Result<f32, String> {
        self.as_f32()
            .ok_or_else(|| format!("expected Float32, got {:?}", self))
    }
    pub fn expect_f64(&self) -> Result<f64, String> {
        self.as_f64()
            .ok_or_else(|| format!("expected Float64, got {:?}", self))
    }
}

impl Debug for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float32(_) => f.write_str("Float32"),
            Self::Float64(_) => f.write_str("Float64"),
        }
    }
}

impl From<f32> for Float {
    fn from(v: f32) -> Self {
        Float::Float32(v)
    }
}
impl From<f64> for Float {
    fn from(v: f64) -> Self {
        Float::Float64(v)
    }
}
