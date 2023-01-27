#[macro_use]
mod macros;

#[macro_use]
mod parse;

#[macro_use]
pub mod tokens;

#[macro_use]
pub mod peek;

pub mod block;
pub mod declr;
pub mod error;
pub mod expr;
pub mod file;
pub mod func;
pub mod ident;
pub mod item;
pub mod jump;
pub mod labeled;
pub mod lit;
pub mod loops;
pub mod op;
pub mod selection;
pub mod specifier;
pub mod stmnt;
pub mod to_tokens;
pub mod ty;

mod buffers;

pub use block::Block;
pub use buffers::{ParseBuffer, TokenStream};
pub use declr::Declr;
pub use error::Error;
pub use expr::Expr;
pub use func::{Fn, FnArgs, FnParam, FnParamNamed, FnParamUnnamed, FnParams};
pub use ident::Ident;
pub use item::Item;
pub use jump::{Break, Continue, Goto, Return};
pub use labeled::Label;
pub use lit::{Lit, LitInt, LitStr};
pub use loops::{DoWhile, For, While};
pub use parse::{Parse, ParseStream, Punctuated};
pub use peek::Peek;
pub use selection::{Case, Default, Else, If, Switch};
pub use stmnt::Stmnt;
pub use to_tokens::ToTokens;
pub use ty::{Ptr, Ty};

pub type Result<T> = std::result::Result<T, Error>;

pub fn parse_file(input: impl AsRef<str>) -> file::File {
    let _input = input.as_ref();
    todo!()
}
