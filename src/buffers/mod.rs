mod parse;

use crate::parse::{Parse, ParseStream};
use crate::peek::Lookahead;
use crate::tokens::{Delimeter, Literal, Punct, TokenCell, TokenTree};
use crate::{Error, Peek, Result};
use std::cell::Cell;
use std::{fmt::Display, marker::PhantomData};

#[derive(Clone)]
pub struct TokenStream {
    entries: Box<[TokenCell]>,
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .entries
                .iter()
                .fold(String::new(), |acc, n| format!("{acc}{} ", n.tt)),
        )
    }
}

impl TokenStream {
    pub fn from_str(str: &str) -> Result<Self> {
        Ok(Self {
            entries: parse::parse_str(str)?,
        })
    }

    pub(crate) fn new_empty() -> Self {
        Self {
            entries: Box::new([]),
        }
    }

    pub(crate) fn new(entries: Box<[TokenCell]>) -> Self {
        Self { entries }
    }

    pub(crate) fn into_inner(self) -> Box<[TokenCell]> {
        self.entries
    }

    pub fn extend(&mut self, ts: &TokenStream) {
        let mut new = self.entries.to_vec();
        new.extend_from_slice(&ts.entries);
        self.entries = new.into_boxed_slice();
    }

    pub fn extend_one(&mut self, tt: TokenTree) {
        let mut new = self.entries.to_vec();
        new.push(TokenCell { col: 0, row: 0, tt });
        self.entries = new.into_boxed_slice();
    }

    pub fn parse<'a, P: Parse>(self) -> Result<P>
    where
        Self: 'a,
    {
        let cursor = self.cursor();
        let parse_buffer = ParseBuffer::new(cursor);
        P::parse(&parse_buffer)
    }

    fn cursor<'a>(&'a self) -> Cursor<'a> {
        let entries = &self.entries;
        unsafe { Cursor::new(entries.as_ptr(), entries.len()) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cursor<'a> {
    pos: *const TokenCell,
    end: *const TokenCell,
    marker: PhantomData<&'a ()>,
}

impl<'a> Cursor<'a> {
    /// # Safety
    /// if len is larger than the referenced slice, it will either
    /// cause undefined behaviour, or othe program bugs.
    /// The resulting cursor might be given a static lifetime.
    pub(crate) unsafe fn new(pos: *const TokenCell, len: usize) -> Self {
        Self {
            pos,
            end: pos.add(len),
            marker: Default::default(),
        }
    }

    pub fn from_token_stream(ts: &'a TokenStream) -> Self {
        Self::from_cursor(ts.cursor())
    }

    pub fn from_cursor<'b>(cursor: Cursor<'b>) -> Self {
        Self {
            marker: Default::default(),
            ..cursor
        }
    }

    pub fn next(self) -> Self {
        Self {
            pos: unsafe { self.pos.add(1) },
            ..self
        }
    }

    fn entry_cell(self) -> Option<&'a TokenCell> {
        if self.pos < self.end {
            let val = unsafe { &*self.pos };
            Some(val)
        } else {
            None
        }
    }

    fn entry(self) -> Option<&'a TokenTree> {
        self.entry_cell().map(|e| &e.tt)
    }

    fn is_empty(&self) -> bool {
        self.pos >= self.end
    }

    pub fn peek<P: Peek>(self) -> bool {
        P::peek(self.clone())
    }

    pub fn lookahead1(self) -> Lookahead<'a> {
        Lookahead::new(self)
    }

    pub fn set(&mut self, other: Self) {
        *self = other;
    }

    pub fn ident(self) -> Option<(&'a String, Cursor<'a>)> {
        match &self.entry()? {
            TokenTree::Ident(ident) => Some((&ident, self.next())),
            _ => None,
        }
    }

    pub fn literal(self) -> Option<(&'a Literal, Cursor<'a>)> {
        match &self.entry()? {
            TokenTree::Literal(lit) => Some((&lit, self.next())),
            _ => None,
        }
    }

    pub fn punct(self) -> Option<(&'a Punct, Cursor<'a>)> {
        match &self.entry()? {
            TokenTree::Punct(punct) => Some((&punct, self.next())),
            _ => None,
        }
    }

    pub fn delim(self) -> Option<(&'a Delimeter, Cursor<'a>)> {
        match &self.entry()? {
            TokenTree::Group(group, _) => Some((group, self.next())),
            _ => None,
        }
    }

    pub fn group(self, delim: Delimeter) -> Option<(&'a TokenStream, Cursor<'a>)> {
        match &self.entry()? {
            TokenTree::Group(cmp, entries) if *cmp == delim => Some((entries, self.next())),
            _ => None,
        }
    }

    pub fn token_tree(self) -> Option<(&'a TokenTree, Cursor<'a>)> {
        if let Some(tt) = self.entry() {
            Some((tt, self.next()))
        } else {
            None
        }
    }

    fn token_cell(self) -> Option<(&'a TokenCell, Cursor<'a>)> {
        if let Some(tt) = self.entry_cell() {
            Some((tt, self.next()))
        } else {
            None
        }
    }

    pub fn token_stream(mut self) -> TokenStream {
        let mut vec = vec![];
        while let Some((tt, next)) = self.token_cell() {
            self = next;
            vec.push(tt.clone());
        }
        TokenStream::new(vec.into_boxed_slice())
    }

    pub fn error(self, err: impl Display) -> Error {
        let entry = self.entry_cell();
        let (col, row) = if let Some(entry) = entry {
            (entry.col, entry.row)
        } else {
            (usize::MAX, usize::MAX)
        };
        Error::new(err.to_string(), None, col, row)
    }
}

pub struct ParseBuffer<'a> {
    cursor: Cell<Cursor<'static>>,
    mark: PhantomData<Cursor<'a>>,
}

impl<'a> ParseBuffer<'a> {
    pub(crate) fn new(cursor: Cursor<'a>) -> Self {
        ParseBuffer {
            cursor: Cell::new(Cursor::from_cursor(cursor)),
            mark: Default::default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cursor().is_empty()
    }

    pub fn fork(&self) -> Self {
        Self {
            cursor: self.cursor.clone(),
            ..*self
        }
    }

    pub fn set(&self, other: Self) {
        self.cursor.set(other.cursor())
    }

    pub fn parse<P: Parse>(&self) -> Result<P> {
        P::parse(self)
    }

    pub fn eat<P: Parse>(&self) -> Option<P> {
        let fork = self.fork();
        if let Ok(parsed) = fork.parse::<P>() {
            self.set(fork);
            Some(parsed)
        } else {
            None
        }
    }

    pub fn expect<P: Parse>(&self) -> bool {
        self.parse::<P>().is_ok()
    }

    pub fn lookahead1(&self) -> Lookahead {
        self.cursor().lookahead1()
    }

    pub fn peek<P: Peek>(&self) -> bool {
        self.cursor().peek::<P>()
    }

    pub fn peek2<P: Peek>(&self) -> bool {
        self.cursor().next().peek::<P>()
    }

    pub fn peek3<P: Peek>(&self) -> bool {
        self.cursor().next().next().peek::<P>()
    }

    pub fn call<P>(&self, f: impl Fn(ParseStream) -> Result<P>) -> Result<P> {
        f(self)
    }

    pub fn step<P>(&self, f: impl Fn(&mut Cursor<'static>) -> Result<P>) -> Result<P> {
        let mut cursor = self.cursor();
        let result = f(&mut cursor)?;
        self.cursor.set(cursor);
        Ok(result)
    }

    pub fn skip(&self) {
        self.cursor.set(self.cursor().next())
    }

    pub fn error(&self, err: impl Display) -> Error {
        self.cursor.get().error(err)
    }

    pub fn cursor(&self) -> Cursor<'static> {
        self.cursor.get()
    }
}
