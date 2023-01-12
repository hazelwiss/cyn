#[macro_use]
mod macros;

#[macro_use]
mod parse;

#[macro_use]
pub mod tokens;

pub mod declr;
pub mod expr;
pub mod file;
pub mod func;
pub mod ident;
pub mod lit;
pub mod op;
pub mod statmnt;
pub mod to_tokens;
pub mod ty;
pub mod var;

mod buffers;

pub use buffers::{ParseBuffer, TokenStream};
pub use expr::Expr;
pub use ident::Ident;
pub use lit::{Lit, LitInt, LitStr};
pub use parse::{Error, Parse, ParseStream, Punctuated, Result};
pub use to_tokens::ToTokens;

pub fn parse_file(input: impl AsRef<str>) -> file::File {
    let _input = input.as_ref();
    todo!()
}
