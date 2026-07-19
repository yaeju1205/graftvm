use std::rc::Rc;

use graftvm_liternal::Liternal;

#[derive(Clone)]
pub struct Fragment {
    pub data: Rc<Liternal>,
    pub alive: bool,
}

impl From<Liternal> for Fragment {
    fn from(data: Liternal) -> Self {
        Self {
            data: Rc::new(data),
            alive: true,
        }
    }
}

impl From<Rc<Liternal>> for Fragment {
    fn from(data: Rc<Liternal>) -> Self {
        Self { data, alive: true }
    }
}
