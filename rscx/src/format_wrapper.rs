use std::fmt::{Display, self};

use crate::render::Render;

pub struct FormatWrapper<T> {
    inner: T,
}

impl<T> FormatWrapper<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Render> Display for FormatWrapper<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.render(f)
    }
}
