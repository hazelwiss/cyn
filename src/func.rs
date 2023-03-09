use crate::tokens::Delimeter;
use crate::{declr::DeclrList, tokens, Block, Ident, Punctuated, Ty};

ast_struct! {
    pub struct Fn {
        pub sign: FnSign,
        pub params: FnParamsOrIdentList,
        pub declr_list: Option<DeclrList>,
        pub body: Block,
    }
}

ast_struct! {
    pub struct FnSign {
        pub ty: Option<Ty>,
        pub ident: Ident,
    }
}

ast_enum! {
    pub enum FnParamsOrIdentList {
        Params(FnParams),
        Ident(FnIdentList),
    }
}

ast_struct! {
    pub struct FnParams {
        pub paren: tokens::Paren,
        pub params: Punctuated<FnParam, token![,]>,
    }
}

ast_enum! {
    pub enum FnParam {
        Named(FnParamNamed),
        Unnamed(Ty),
    }
}

ast_struct! {
    pub struct FnParamNamed {
        pub ty: Ty,
        pub ident: Ident,
    }
}

ast_struct! {
    pub struct FnIdentList {
        pub paren: tokens::Paren,
        pub idents: Punctuated<Ident, token![,]>,
    }
}

ast_struct! {
    pub struct FnArgs {
        pub paren: tokens::Paren,
        pub args: Punctuated<Expr, token![,]>,
    }
}

use crate::{Declr, Expr, Parse, ParseStream, Result};

impl Parse for Fn {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            sign: parse.parse()?,
            params: parse.parse()?,
            declr_list: parse.parse()?,
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

impl Parse for FnParamsOrIdentList {
    fn parse(parse: ParseStream) -> Result<Self> {
        let fork = parse.fork();
        fork.step(|cursor| {
            if let Some((_, next)) = cursor.group(Delimeter::Paren) {
                cursor.set(next);
                Ok(())
            } else {
                Err(cursor.error("expected parantheses"))
            }
        })?;
        Ok(if fork.expect::<Declr>() {
            Self::Ident(parse.parse()?)
        } else {
            Self::Params(parse.parse()?)
        })
    }
}

impl Parse for FnParams {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in parse)?,
            params: content.call(Punctuated::parse_non_terminated)?,
        })
    }
}

impl Parse for FnParam {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(if parse.fork().expect::<Ty>() {
            let ty = parse.parse()?;
            if parse.peek::<Ident>() {
                Self::Named(FnParamNamed {
                    ty,
                    ident: parse.parse()?,
                })
            } else {
                Self::Unnamed(ty)
            }
        } else {
            return Err(parse.error("expected type"));
        })
    }
}

impl Parse for FnParamNamed {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            ty: parse.parse()?,
            ident: parse.parse()?,
        })
    }
}

impl Parse for FnIdentList {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in parse)?,
            idents: content.call(Punctuated::parse_non_terminated)?,
        })
    }
}

impl Parse for FnArgs {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in parse)?,
            args: content.call(Punctuated::parse_non_terminated)?,
        })
    }
}

mod quote {
    use super::*;
    use crate::{to_tokens, ToTokens, TokenStream};

    impl ToTokens for Fn {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                sign,
                params,
                declr_list,
                body,
            } = self;
            sign.to_tokens(tokens);
            params.to_tokens(tokens);
            declr_list.to_tokens(tokens);
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

    impl ToTokens for FnParamsOrIdentList {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                FnParamsOrIdentList::Params(e) => e.to_tokens(tokens),
                FnParamsOrIdentList::Ident(e) => e.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for FnParams {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { paren: _, params } = self;
            to_tokens::parenthesized(params).to_tokens(tokens);
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

    impl ToTokens for FnParamNamed {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { ty, ident } = self;
            ty.to_tokens(tokens);
            ident.to_tokens(tokens);
        }
    }

    impl ToTokens for FnIdentList {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { paren: _, idents } = self;
            to_tokens::parenthesized(idents).to_tokens(tokens);
        }
    }

    impl ToTokens for FnArgs {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { paren: _, args } = self;
            to_tokens::parenthesized(args).to_tokens(tokens);
        }
    }
}
