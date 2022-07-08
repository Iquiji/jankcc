use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use crate::lexer::OriginalLocation;

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct Spanned<T>
where
    T: Clone + Debug,
{
    pub(crate) inner: T,
    #[serde(skip_serializing)]
    #[serde(default)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl Default for Span {
    fn default() -> Self {
        Self {
            start: OriginalLocation {
                file: String::new(),
                line: 0,
                collumn: 0,
            },
            end: OriginalLocation {
                file: String::new(),
                line: 0,
                collumn: 0,
            },
        }
    }
}

impl<T: Clone + Debug + std::cmp::PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
