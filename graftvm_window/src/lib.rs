use std::rc::Rc;

use graftvm_liternal::Liternal;

#[derive(Clone)]
pub struct WindowSlot {
    pub data: Rc<Liternal>,
    pub alive: bool,
}

impl From<Liternal> for WindowSlot {
    fn from(data: Liternal) -> Self {
        Self {
            data: Rc::new(data),
            alive: true,
        }
    }
}

impl From<Rc<Liternal>> for WindowSlot {
    fn from(data: Rc<Liternal>) -> Self {
        Self { data, alive: true }
    }
}

pub struct WindowState {
    pub id: usize,
    pub active: bool,
    pub frozen: bool,
    pub closed: bool,
}

impl WindowState {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            active: true,
            frozen: false,
            closed: false,
        }
    }
}

pub struct Window {
    pub slots: Vec<Option<WindowSlot>>,
    pub state: WindowState,
}

impl Window {
    pub fn new(id: usize) -> Self {
        Self {
            slots: Vec::new(),
            state: WindowState::new(id),
        }
    }
}
