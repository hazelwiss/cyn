use crate::{Expr, Ident, Parse, ParseStream, Punctuated, Result};

ast_struct! {
    pub struct Vars {
        pub vars: Punctuated<Var, token![,]>
    }
}

ast_enum! {
    pub enum Var{
        Init(VarInit),
        Uninit(VarUninit),
    }
}

ast_struct! {
    pub struct VarInit {
        pub ident: Ident,
        pub assign: token![=],
        pub expr: Box<Expr>,
    }
}

ast_struct! {
    pub struct VarUninit {
        pub ident: Ident,
    }
}

impl Parse for Vars {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            vars: Punctuated::parse_non_terminated(parse)?,
        })
    }
}

impl Parse for Var {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(if parse.lookahead1::<token![=]>() {
            Self::Init(parse.parse()?)
        } else {
            Self::Uninit(parse.parse()?)
        })
    }
}

impl Parse for VarUninit {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            ident: parse.parse()?,
        })
    }
}

impl Parse for VarInit {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            ident: parse.parse()?,
            assign: parse.parse()?,
            expr: Box::new(parse.parse()?),
        })
    }
}

mod quote {
    use super::*;
    use crate::ToTokens;

    impl ToTokens for Vars {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            self.vars.to_tokens(tokens)
        }
    }

    impl ToTokens for Var {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            match self {
                Var::Init(var) => var.to_tokens(tokens),
                Var::Uninit(var) => var.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for VarUninit {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self { ident } = self;
            ident.to_tokens(tokens)
        }
    }

    impl ToTokens for VarInit {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self {
                ident,
                assign,
                expr,
            } = self;
            ident.to_tokens(tokens);
            assign.to_tokens(tokens);
            expr.to_tokens(tokens);
        }
    }
}
