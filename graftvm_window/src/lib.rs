use graftvm_fragment::Fragment;

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
    pub slots: Vec<Option<Fragment>>,
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
