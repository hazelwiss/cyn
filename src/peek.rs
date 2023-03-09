use crate::buffers::Cursor;
use crate::tokens::Token;
use crate::Error;
use std::cell::RefCell;
use std::marker::PhantomData;

pub trait Peek {
    fn peek<'a>(cursor: Cursor<'a>) -> bool;

    fn display() -> &'static str;
}

impl<T: Token> Peek for T {
    fn peek<'a>(cursor: Cursor<'a>) -> bool {
        <T as Token>::peek(cursor)
    }

    fn display() -> &'static str {
        <T as Token>::display()
    }
}

pub struct Lookahead<'a> {
    cursor: Cursor<'static>,
    error_vec: RefCell<Vec<&'static str>>,
    mark: PhantomData<Cursor<'a>>,
}

impl<'a> Lookahead<'a> {
    pub(crate) fn new(cursor: Cursor<'static>) -> Self {
        Self {
            cursor,
            error_vec: RefCell::new(vec![]),
            mark: Default::default(),
        }
    }

    pub fn peek<P: Peek>(&self) -> bool {
        if P::peek(self.cursor) {
            true
        } else {
            self.error_vec.borrow_mut().push(P::display());
            false
        }
    }

    pub fn error(&self) -> Error<'static> {
        self.cursor
            .error(format!("expected any of {:?}", self.error_vec))
    }
}
