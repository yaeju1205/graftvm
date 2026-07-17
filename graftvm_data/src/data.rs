#[derive(Clone)]
pub enum Int {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
}

#[derive(Clone)]
pub enum UInt {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
}

#[derive(Clone)]
pub enum Float {
    Float32(f32),
    Float64(f64),
}

#[derive(Clone)]
pub enum Liternal {
    Int(Int),
    UInt(UInt),
    Float(Float),
    String(String),
    Bool(bool),
}
