use crate::Ident;

ast_struct! {
    pub struct Ty{
        pub ident: Ident,
    }
}

use crate::Parse;

impl Parse for Ty {
    fn parse(parse: crate::ParseStream) -> crate::Result<Self> {
        Ok(Self {
            ident: parse.parse()?,
        })
    }
}

mod quote {
    use super::Ty;
    use crate::{ToTokens, TokenStream};

    impl ToTokens for Ty {
        fn to_tokens(&self, extend: &mut TokenStream) {
            let Self { ident } = self;
            ident.to_tokens(extend)
        }
    }
}
