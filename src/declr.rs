ast_enum! {
    pub enum Declr{
        Var(DeclrVar),
    }
}

use crate::ty::Ty;
use crate::var::Vars;

ast_struct! {
    pub struct DeclrVar{
        pub ty: Ty,
        pub vars: Vars,
        pub semi_colon: token![;],
    }
}

use crate::{Parse, ParseStream, Result};

impl Parse for Declr {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self::Var(parse.parse()?))
    }
}

impl Parse for DeclrVar {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            ty: parse.parse()?,
            vars: parse.parse()?,
            semi_colon: parse.parse()?,
        })
    }
}

mod quote {
    use super::{Declr, DeclrVar};
    use crate::{ToTokens, TokenStream};

    impl ToTokens for Declr {
        fn to_tokens(&self, extend: &mut TokenStream) {
            match self {
                Declr::Var(v) => v.to_tokens(extend),
            }
        }
    }

    impl ToTokens for DeclrVar {
        fn to_tokens(&self, extend: &mut TokenStream) {
            let Self {
                ty,
                vars,
                semi_colon,
            } = self;
            ty.to_tokens(extend);
            vars.to_tokens(extend);
            semi_colon.to_tokens(extend);
        }
    }
}
