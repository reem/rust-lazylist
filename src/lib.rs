#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(test, feature(core))]

//! # lazylist
//!
//! A lazy, reference counted linked list similar to Haskell's [].

#[macro_use(lazy)]
extern crate lazy;

use lazy::single::Thunk;

use std::rc::Rc;
use std::iter::FromIterator;

#[macro_export]
macro_rules! list { ($val:expr) => { Rc::new(lazy!($val)) } }

#[macro_export]
macro_rules! pair { ($val:expr, $list:expr) => { list!($crate::List::Cons($val, $list)) } }

#[macro_export]
macro_rules! nil { () => { list!($crate::List::Nil) } }

/// A lazy, reference counted, singly linked list.
///
/// See `List` for methods.
pub type RcList<T> = Rc<Thunk<List<T>>>;

use List::{Nil, Cons};

/// A Node in a lazy, reference counted, singly linked list.
#[derive(Clone)]
pub enum List<T: 'static> {
    Nil,
    Cons(T, RcList<T>)
}

impl<T: 'static> List<T> {
    pub fn new() -> RcList<T> {
        nil!()
    }

    pub fn singleton(val: T) -> RcList<T> {
        pair!(val, nil!())
    }

    pub fn head(&self) -> Option<&T> {
        match *self {
            Cons(ref val, _) => Some(val),
            Nil => None
        }
    }

    pub fn tail(&self) -> Option<RcList<T>> {
        match *self {
            Cons(_, ref tail) => Some(tail.clone()),
            Nil => None
        }
    }
}

pub trait RcListMethods<T> {
    fn push(self, val: T) -> RcList<T>;
    fn pop(&self) -> Option<(&T, RcList<T>)>;
    fn len(&self) -> usize;
}

impl<T: 'static> RcListMethods<T> for RcList<T> {
    fn push(self, val: T) -> RcList<T> {
        pair!(val, self)
    }

    fn pop(&self) -> Option<(&T, RcList<T>)> {
        self.tail().and_then(|next| self.head().map(|head| {
            (head, next)
        }))
    }

    fn len(&self) -> usize {
        self.count()
    }
}

impl<T> FromIterator<T> for RcList<T> {
    fn from_iter<I>(mut iter: I) -> RcList<T>
    where I: Iterator<Item=T> + 'static {
        list!({
            match iter.next() {
                Some(val) => Cons(val, FromIterator::from_iter(iter)),
                None => Nil
            }
        })
    }
}

impl<'a, T> Iterator for &'a RcList<T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let (value, rest) = match ****self {
            Cons(ref value, ref rest) => (value, rest),
            Nil => return None
        };

        *self = rest;
        Some(value)
    }
}

#[test]
fn test_fib() {
    fn fib(n: u64) -> u64 {
        let mut n0 = 0;
        let mut n1 = 1;

        for _ in 0..n {
            let sum = n0 + n1;
            n0 = n1;
            n1 = sum;
        }

        return n0;
    }

    fn fibs() -> RcList<u64> {
        fn fibs_inner(n0: u64, n1: u64) -> RcList<u64> {
            pair!(n0, fibs_inner(n1, n0 + n1))
        }

        fibs_inner(0, 1)
    }

    for (i, &x) in fibs().take(100).enumerate() {
        assert_eq!(x, fib(i as u64))
    }
}

