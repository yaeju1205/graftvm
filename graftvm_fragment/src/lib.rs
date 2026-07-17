use std::rc::Rc;

use graftvm_data::{data::Liternal, kind::Kind};

#[derive(Clone)]
pub struct Fragment {
    pub data: Rc<Liternal>,
    pub kind: Kind,
    pub alive: bool,
}

impl Fragment {
    pub fn is_int(&self) -> bool {
        self.kind == Kind::Int
    }

    pub fn is_uint(&self) -> bool {
        self.kind == Kind::UInt
    }

    pub fn is_float(&self) -> bool {
        self.kind == Kind::Float
    }

    pub fn is_string(&self) -> bool {
        self.kind == Kind::String
    }

    pub fn is_bool(&self) -> bool {
        self.kind == Kind::Bool
    }
}

impl From<Liternal> for Fragment {
    fn from(data: Liternal) -> Self {
        let kind = Kind::from(&data);
        let data = Rc::new(data);

        Self {
            data,
            kind,
            alive: true,
        }
    }
}

impl From<Rc<Liternal>> for Fragment {
    fn from(data: Rc<Liternal>) -> Self {
        let kind = Kind::from(data.as_ref());

        Self {
            data,
            kind,
            alive: true,
        }
    }
}
