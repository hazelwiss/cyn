use crate::parse::{Parse, ParseStream, Result};

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

impl Parse for Ident {
    fn parse(parse: ParseStream) -> Result<Self> {
        let ident = parse.ident().ok_or(parse.error("expected identifier"))?;
        let err = &format!("invalid identifier {ident}");
        Ok(Ident::new(ident).ok_or(parse.error(err))?)
    }
}

mod quote {
    use super::Ident;
    use crate::tokens::{TokenTree, TokenTreeTy};
    use crate::{ToTokens, TokenStream};

    impl ToTokens for Ident {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend_one(TokenTree {
                col: 0,
                row: 0,
                ty: TokenTreeTy::Ident(self.0.clone()),
            })
        }
    }
}
