use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Location {
    pub offset: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span {
    pub file_id: usize,
    pub from: Location,
    pub to: Location,
}

impl Span {
    pub fn combine(start: &Span, end: &Span) -> Span {
        assert!(start.file_id == end.file_id);
        Span {
            file_id: start.file_id,
            from: start.from,
            to: end.to,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> fmt::Display for Spanned<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.node)
    }
}

impl<T> std::error::Error for Spanned<T> where T: std::error::Error {}

pub trait Spanning
where
    Self: Sized,
{
    fn spanning(self, span: Span) -> Spanned<Self>;
}

#[macro_export]
macro_rules! impl_spanning {
    ($impl_type:ty) => {
        impl<'a> Spanning for $impl_type {
            fn spanning(self, span: Span) -> Spanned<$impl_type> {
                Spanned { node: self, span }
            }
        }
    };
}

impl_spanning!(String);
impl_spanning!(u64);
impl_spanning!(usize);
impl_spanning!(&'a str);
