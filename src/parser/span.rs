use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::lexer::OriginalLocation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Spanned<T>
where
    T: Clone + Debug,
{
    inner: T,
    span: Span,
}
impl<T: Clone + Debug> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: Clone + Debug> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Span {
    start: OriginalLocation,
    end: OriginalLocation,
}
impl Span {
    pub(crate) fn new(start: OriginalLocation, end: OriginalLocation) -> Self {
        Span { start, end }
    }
}
