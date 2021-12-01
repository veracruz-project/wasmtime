#![allow(missing_docs)]

#[derive(Debug)]
pub struct Backtrace;

impl Backtrace {
    pub fn new_unresolved() -> Self {
        Self
    }
}

impl From<Vec<()>> for Backtrace {
    fn from(_: Vec<()>) -> Self {
        Self
    }
}

