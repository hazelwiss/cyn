use crate::buffers::ParseBuffer;
use crate::{Peek, Result};
use std::fmt::Debug;
use std::marker::PhantomData;

pub type ParseStream<'a> = &'a ParseBuffer<'a>;

pub trait Parse: Sized {
    fn parse(parse: ParseStream) -> Result<Self>;
}

impl<T: Parse> Parse for Option<T> {
    fn parse(parse: ParseStream) -> Result<Self> {
        let fork = parse.fork();
        if let Ok(parsed) = fork.parse::<T>() {
            parse.set(fork);
            Ok(Some(parsed))
        } else {
            Ok(None)
        }
    }
}

impl<T: Parse> Parse for Box<T> {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self::new(T::parse(parse)?))
    }
}

#[derive(Debug)]
pub struct Punctuated<T, P> {
    punctuated: Vec<T>,
    mark: PhantomData<P>,
    terminated: bool,
}

impl<T: Parse, P: Peek + Parse> Punctuated<T, P> {
    pub fn parse_non_terminated(parse: ParseStream) -> Result<Self> {
        let mut vec = vec![];
        if !parse.is_empty() {
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

    pub fn len(&self) -> usize {
        self.punctuated.len()
    }
}

#[macro_export]
macro_rules! delim {
    ($ty:ident, $out:ident in $parse:expr) => {{
        $out = {
            let ts = $crate::tokens::$ty::parse_inner($parse)?;
            let cursor = $crate::buffers::Cursor::from_token_stream(&ts);
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

pub fn parse_into_vec<T: Parse>(parse: ParseStream) -> Vec<T> {
    let mut vec = vec![];
    loop {
        let fork = parse.fork();
        if let Ok(p) = fork.parse::<T>() {
            parse.set(fork);
            vec.push(p);
        } else {
            break;
        }
    }
    vec
}

mod quote {
    use super::*;
    use crate::ToTokens;

    impl<T: ToTokens, P: Default + ToTokens> ToTokens for Punctuated<T, P> {
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
