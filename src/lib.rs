#![feature(if_let_guard)]

#[macro_use]
mod macros;

#[macro_use]
pub mod tokens;

pub mod declarations;
pub mod expressions;
pub mod file;
pub mod ident;
pub mod lit;
pub mod op;
pub mod statements;
pub mod ty;

mod buffers;
mod parse;

pub use buffers::{ParseBuffer, TokenStream};
pub use ident::Ident;
pub use lit::{Lit, LitInt, LitStr};
pub use parse::{Error, Parse, ParseStream, Punctuated, Result};
pub use tokens::ToTokens;

pub fn to_tokens<T: ToTokens>(v: &T) -> TokenStream {
    let mut ts = TokenStream::new_empty();
    v.to_tokens(&mut ts);
    ts
}

pub fn parse_file(input: impl AsRef<str>) -> file::File {
    let _input = input.as_ref();
    todo!()
}
