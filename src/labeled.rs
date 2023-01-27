use crate::{Ident, Stmnt};

ast_struct! {
    pub struct Label {
        pub ident: Ident,
        pub colon: token![:],
        pub stmnt: Box<Stmnt>,
    }
}

use crate::{Parse, ParseStream, Result};

impl Parse for Label {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            ident: parse.parse()?,
            colon: parse.parse()?,
            stmnt: parse.parse()?,
        })
    }
}

mod quote {
    use super::Label;
    use crate::{ToTokens, TokenStream};

    impl ToTokens for Label {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                ident,
                colon,
                stmnt,
            } = self;
            ident.to_tokens(tokens);
            colon.to_tokens(tokens);
            stmnt.to_tokens(tokens);
        }
    }
}
