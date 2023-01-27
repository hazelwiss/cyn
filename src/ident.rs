use crate::buffers::Cursor;
use crate::tokens::Token;

#[derive(Debug, Clone)]
pub struct Ident(String);

impl Ident {
    pub fn new(ident: String) -> Option<Self> {
        if Self::valid_ident(&ident) {
            Some(Self(ident))
        } else {
            None
        }
    }

    fn valid_ident(ident: &str) -> bool {
        match ident {
            "while" | "if" | "for" => false,
            _ => true,
        }
    }
}

impl Token for Ident {
    fn peek(cursor: Cursor) -> bool {
        if let Some((ident, _)) = cursor.ident() {
            Ident::valid_ident(&ident)
        } else {
            false
        }
    }

    fn display() -> &'static str {
        "identifier"
    }
}

use crate::{Parse, ParseStream, Result};

impl Parse for Ident {
    fn parse(parse: ParseStream) -> Result<Self> {
        parse.step(|cursor| {
            if let Some((ident, new)) = cursor.ident() {
                cursor.set(new);
                Ok(Ident::new(ident.clone())
                    .ok_or(parse.error(format!("invalid identifier {ident}")))?)
            } else {
                Err(parse.error("expected identifier"))
            }
        })
    }
}

mod quote {
    use super::Ident;
    use crate::tokens::TokenTree;
    use crate::{ToTokens, TokenStream};

    impl ToTokens for Ident {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend_one(TokenTree::Ident(self.0.clone()))
        }
    }
}
