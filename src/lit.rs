ast_enum! {
    pub enum Lit{
        Int(LitInt),
        Str(LitStr),
    }
}

ast_struct! {
    pub struct LitInt{
        value: i128,
    }
}

ast_struct! {
    pub struct LitStr{
        str: String,
    }
}

use crate::buffers::Cursor;
use crate::tokens::Token;

impl Token for Lit {
    fn peek(cursor: Cursor) -> bool {
        cursor.literal().is_some()
    }

    fn display() -> &'static str {
        "literal"
    }
}

use crate::tokens::Literal;
use crate::{Parse, ParseStream, Result};

impl Parse for Lit {
    fn parse(parse: ParseStream) -> Result<Self> {
        parse.step(|cursor| {
            if let Some((lit, rest)) = cursor.literal() {
                cursor.set(rest);
                Ok(match lit.clone() {
                    Literal::Str(str) => Self::Str(LitStr { str }),
                    Literal::Int(value) => Self::Int(LitInt { value }),
                    Literal::Float(_) => unimplemented!(),
                })
            } else {
                Err(parse.error("expected literal"))
            }
        })
    }
}

impl Parse for LitInt {
    fn parse(parse: ParseStream) -> Result<Self> {
        parse.step(|cursor| match cursor.literal() {
            Some((Literal::Int(int), next)) => {
                cursor.set(next);
                Ok(Self { value: *int })
            }
            _ => Err(parse.error("expected integer literal")),
        })
    }
}

impl Parse for LitStr {
    fn parse(parse: ParseStream) -> Result<Self> {
        parse.step(|cursor| match cursor.literal() {
            Some((Literal::Str(str), next)) => {
                cursor.set(next);
                Ok(Self { str: str.clone() })
            }
            _ => Err(parse.error("expected integer literal")),
        })
    }
}

mod quote {
    use super::{Lit, LitInt, LitStr};
    use crate::tokens::{Literal, TokenTree};
    use crate::{ToTokens, TokenStream};

    impl ToTokens for Lit {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                Lit::Int(int) => int.to_tokens(tokens),
                Lit::Str(str) => str.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for LitInt {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend_one(TokenTree::Literal(Literal::Int(self.value)));
        }
    }

    impl ToTokens for LitStr {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend_one(TokenTree::Literal(Literal::Str(self.str.clone())));
        }
    }
}
