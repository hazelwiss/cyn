use crate::{tokens, Block, Ident, Punctuated, Ty};

ast_struct! {
    pub struct Fn {
        pub sign: FnSign,
        pub params: FnParams,
        pub body: Block,
    }
}

ast_struct! {
    pub struct FnSign {
        pub ty: Ty,
        pub ident: Ident,
    }
}

ast_enum! {
    pub enum FnParam {
        Named(FnParamNamed),
        Unnamed(FnParamUnnamed),
    }
}

ast_struct! {
    pub struct FnParamUnnamed {
        pub ty: Ty,
    }
}

ast_struct! {
    pub struct FnParamNamed {
        pub ty: Ty,
        pub ident: Ident,
    }
}

ast_struct! {
    pub struct FnParams {
        pub paren: tokens::Paren,
        pub params: Punctuated<FnParam, token![,]>,
    }
}

ast_struct! {
    pub struct FnArgs {
        pub paren: tokens::Paren,
        pub args: Punctuated<Ident, token![,]>,
    }
}

use crate::{Parse, ParseStream, Result};

impl Parse for Fn {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            sign: parse.parse()?,
            params: parse.parse()?,
            body: parse.parse()?,
        })
    }
}

impl Parse for FnSign {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            ty: parse.parse()?,
            ident: parse.parse()?,
        })
    }
}

impl Parse for FnParam {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(if let Ok(ty) = parse.parse::<Ty>() {
            if parse.peek::<Ident>() {
                Self::Named(FnParamNamed {
                    ty,
                    ident: parse.parse()?,
                })
            } else {
                Self::Unnamed(FnParamUnnamed { ty })
            }
        } else {
            return Err(parse.error("expected type"));
        })
    }
}

impl Parse for FnParams {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in parse)?,
            params: Punctuated::parse_non_terminated(&content)?,
        })
    }
}

impl Parse for FnArgs {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in parse)?,
            args: Punctuated::parse_non_terminated(&content)?,
        })
    }
}

mod quote {
    use super::*;
    use crate::{to_tokens, ToTokens, TokenStream};

    impl ToTokens for Fn {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { sign, params, body } = self;
            sign.to_tokens(tokens);
            params.to_tokens(tokens);
            body.to_tokens(tokens);
        }
    }

    impl ToTokens for FnSign {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { ty, ident } = self;
            ty.to_tokens(tokens);
            ident.to_tokens(tokens);
        }
    }

    impl ToTokens for FnParamUnnamed {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { ty } = self;
            ty.to_tokens(tokens);
        }
    }

    impl ToTokens for FnParamNamed {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { ty, ident } = self;
            ty.to_tokens(tokens);
            ident.to_tokens(tokens);
        }
    }

    impl ToTokens for FnParam {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                FnParam::Named(named) => named.to_tokens(tokens),
                FnParam::Unnamed(unnamed) => unnamed.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for FnParams {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { paren: _, params } = self;
            to_tokens::parenthesized(params).to_tokens(tokens);
        }
    }

    impl ToTokens for FnArgs {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { paren: _, args } = self;
            to_tokens::parenthesized(args).to_tokens(tokens);
        }
    }
}
