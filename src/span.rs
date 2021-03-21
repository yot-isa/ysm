use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Location {
    pub offset: usize,
}

impl Location {
    pub fn new() -> Location {
        Location { offset: 0 }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span {
    pub from: Location,
    pub to: Location,
}

impl Span {
    pub fn new() -> Span {
        Span {
            from: Location::new(),
            to: Location::new(),
        }
    }

    pub fn combine(start: &Span, end: &Span) -> Span {
        Span {
            from: start.from,
            to: end.to,
        }
    }

    pub fn between(start: &Span, end: &Span) -> Span {
        Span {
            from: start.to,
            to: end.from,
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::new()
    }
}

impl From<(usize, usize)> for Span {
    fn from(tuple: (usize, usize)) -> Span {
        Span {
            from: Location { offset: tuple.0 },
            to: Location { offset: tuple.1 },
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
