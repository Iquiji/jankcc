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
    pub(crate) inner: T,
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

impl<T: Clone + Debug> Spanned<T> {
    pub(crate) fn new(value: T, start: OriginalLocation, end: OriginalLocation) -> Spanned<T> {
        Self {
            inner: value,
            span: Span::new(start, end),
        }
    }
    pub(crate) fn boxed_new(
        value: T,
        start: OriginalLocation,
        end: OriginalLocation,
    ) -> Box<Spanned<T>> {
        Box::new(Self {
            inner: value,
            span: Span::new(start, end),
        })
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

impl<T: Clone + Debug> Spanned<T> {
    pub(crate) fn error_unexpected_span(&mut self, found: Spanned<T>, expected: &str) {
        unimplemented!()
    }
}
