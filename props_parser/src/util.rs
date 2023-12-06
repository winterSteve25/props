use std::cmp::Ordering;
use std::fmt::{Debug, Display, format, Formatter};
use std::rc::Rc;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Access<T> {
    Rc(Rc<T>),
    Owned(T)
}

impl <T> From<Rc<T>> for Access<T> {
    fn from(value: Rc<T>) -> Self {
        Access::Rc(value.clone())
    }
}

impl <T> From<&Rc<T>> for Access<T> {
    fn from(value: &Rc<T>) -> Self {
        Access::Rc(value.clone())
    }
}

impl <T> From<T> for Access<T> {
    fn from(value: T) -> Self {
        Access::Owned(value)
    }
}

impl<T: PartialOrd> PartialOrd for Access<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Access::Rc(rc1), Access::Rc(rc2)) => rc1.partial_cmp(rc2),
            (Access::Rc(rc), Access::Owned(v)) => rc.as_ref().partial_cmp(v),
            (Access::Owned(v), Access::Rc(rc)) => v.partial_cmp(rc.as_ref()),
            (Access::Owned(v1), Access::Owned(v2)) => v1.partial_cmp(v2),
        }
    }
}

impl<T> Access<T> {
    pub fn map<R, F: Fn(&T) -> R>(&self, f: F) -> R {
        match self {
            Access::Rc(rc) => f(rc.as_ref()),
            Access::Owned(v) => f(v)
        }
    }
}

impl<T: Debug> Display for Access<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Access::Rc(rc) => write!(f, "{:?}", rc),
            Access::Owned(v) => write!(f, "{:?}", v),
        }
    }
}