use crate::buffers::ParseBuffer;
use crate::tokens::Token;
use std::fmt::{self, Debug, Display};
use std::marker::PhantomData;
use std::path::PathBuf;

pub type ParseStream<'a> = &'a ParseBuffer<'a>;

#[derive(Debug)]
pub struct Error {
    msg: String,
    file: Option<PathBuf>,
    col: usize,
    row: usize,
}

impl Error {
    pub fn new(msg: impl Display, file: Option<PathBuf>, col: usize, row: usize) -> Self {
        Self {
            msg: msg.to_string(),
            file,
            col,
            row,
        }
    }

    pub fn msg(&self) -> &String {
        &self.msg
    }

    pub fn file(&self) -> &Option<PathBuf> {
        &self.file
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn row(&self) -> usize {
        self.row
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            msg,
            file,
            col,
            row,
        } = self;
        let msg = format!("{row}:{col} {msg}");
        f.write_str(&match file {
            Some(file) => {
                if let Some(file) = file.to_str() {
                    format!("{file} {msg}")
                } else {
                    msg
                }
            }
            _ => msg,
        })
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Parse: Sized {
    fn parse(parse: ParseStream) -> Result<Self>;
}

impl<T: Parse> Parse for Option<T> {
    fn parse(parse: ParseStream) -> Result<Self> {
        let fork = parse.fork();
        if let Ok(parsed) = fork.parse::<T>() {
            parse.update_cursor(fork.cursor());
            Ok(Some(parsed))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub struct Punctuated<T, P: Token> {
    punctuated: Vec<T>,
    mark: PhantomData<P>,
    terminated: bool,
}

impl<T: Parse, P: Token> Punctuated<T, P> {
    pub fn parse_non_terminated(parse: ParseStream) -> Result<Self> {
        let mut vec = vec![];
        if parse.peek::<T>() {
            loop {
                vec.push(parse.parse()?);
                if parse.peek::<P>() {
                    parse.parse::<P>()?;
                } else {
                    break;
                }
            }
        }
        Ok(Self {
            punctuated: vec,
            mark: Default::default(),
            terminated: false,
        })
    }

    pub fn parse_terminated(_parse: ParseStream) -> Result<Self> {
        unimplemented!()
    }
}

#[macro_export]
macro_rules! delim {
    ($ty:ident, $out:ident in $parse:expr) => {{
        $out = {
            let parse = $crate::tokens::$ty::parse_inner($parse)?;
            let cursor = unsafe { $crate::buffers::Cursor::new(parse.as_ptr(), parse.len()) };
            $crate::buffers::ParseBuffer::new(cursor)
        };
        Ok($crate::tokens::$ty)
    }};
}

#[macro_export]
macro_rules! braced {
    ($out:ident in $parse:expr) => {
        $crate::delim!(Brace, $out in $parse)
    };
}

#[macro_export]
macro_rules! parenthesized {
    ($out:ident in $parse:expr) => {
        $crate::delim!(Paren, $out in $parse)
    };
}

#[macro_export]
macro_rules! bracketed {
    ($out:ident in $parse:expr) => {
        $crate::delim!(Bracket, $out in $parse)
    };
}

mod quote {
    use super::*;
    use crate::ToTokens;

    impl<T: ToTokens, P: Token + ToTokens> ToTokens for Punctuated<T, P> {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let p = P::default();
            let mut iter = self.punctuated.iter();
            let Some(e) = iter.next() else {
                return;
            };
            e.to_tokens(tokens);
            while let Some(e) = iter.next() {
                p.to_tokens(tokens);
                e.to_tokens(tokens)
            }
            if self.terminated {
                p.to_tokens(tokens);
            }
        }
    }
}
