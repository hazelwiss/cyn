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

use crate::tokens::Literal;
use crate::{Parse, ParseStream, Result};

impl Parse for Lit {
    fn parse(parse: ParseStream) -> Result<Self> {
        if let Some(lit) = parse.fork().literal() {
            Ok(match lit {
                Literal::Str(_) => Self::Str(parse.parse()?),
                Literal::Int(_) => Self::Int(parse.parse()?),
                Literal::Float(_) => todo!(),
            })
        } else {
            Err(parse.error("expected literal"))
        }
    }
}

impl Parse for LitInt {
    fn parse(parse: ParseStream) -> Result<Self> {
        if let Some(Literal::Int(value)) = parse.literal() {
            Ok(Self { value })
        } else {
            Err(parse.error("expected integer literal"))
        }
    }
}

impl Parse for LitStr {
    fn parse(parse: ParseStream) -> Result<Self> {
        if let Some(Literal::Str(str)) = parse.literal() {
            Ok(Self { str })
        } else {
            Err(parse.error("expected string literal"))
        }
    }
}

mod quote {
    use super::*;
    use crate::tokens::{Literal, TokenTree, TokenTreeTy};
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
            tokens.extend_one(TokenTree {
                col: 0,
                row: 0,
                ty: TokenTreeTy::Literal(Literal::Int(self.value)),
            });
        }
    }

    impl ToTokens for LitStr {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend_one(TokenTree {
                col: 0,
                row: 0,
                ty: TokenTreeTy::Literal(Literal::Str(self.str.clone())),
            });
        }
    }
}
