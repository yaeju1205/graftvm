use graftvm_liternal::Liternal;

#[derive(Clone)]
pub struct WindowSlot {
    pub data: Liternal,
}

impl From<Liternal> for WindowSlot {
    fn from(data: Liternal) -> Self {
        Self { data }
    }
}

pub struct Window {
    pub slots: Vec<Option<WindowSlot>>,
    pub id: usize,
}

impl Window {
    pub fn new(id: usize) -> Self {
        Self {
            slots: Vec::new(),
            id,
        }
    }
}
