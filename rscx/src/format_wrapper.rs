use std::fmt::{Display, self};

use crate::renderable::Renderable;

pub struct FormatWrapper<T: Renderable> {
    inner: T,
}

impl<T: Renderable> FormatWrapper<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Renderable> Display for FormatWrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.render(f)
    }
}
