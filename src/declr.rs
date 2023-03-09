use crate::expr;
use crate::specifier::TypeQualifierList;
use crate::{tokens, Expr, FnArgs, FnParams, Ident, Ptr, Punctuated, Ty};

ast_struct! {
    pub struct Declr {
        pub ty: Ty,
        pub vars: Punctuated<InitDeclarator, token![,]>,
        pub semi_colon: token![;],
    }
}

ast_struct! {
    pub struct DeclrList {
        pub items: Vec<Declr>,
    }
}

ast_enum! {
    pub enum InitDeclarator {
        Init(InitDeclaratorInit),
        Uninit(InitDeclaratorUninit),
    }
}

ast_struct! {
    pub struct InitDeclaratorInit {
        pub declarator: Declarator,
        pub assign_tk: token![=],
        pub expr: Box<Expr>,
    }
}

ast_struct! {
    pub struct InitDeclaratorUninit {
        pub declarator: Declarator,
    }
}

ast_struct! {
    pub struct Declarator {
        pub ptr: Option<Ptr>,
        pub direct_declarator: DirectDeclarator,
    }
}

ast_enum! {
    pub enum DirectDeclarator {
        Ident(Ident),
        Paren(DDParen),
        Bracket(DDBracket),
        Params(DDParams),
        Args(DDArgs)
    }
}

ast_struct! {
    pub struct DDParen {
        pub paren: tokens::Paren,
        pub declarator: Box<Declarator>
    }
}

ast_struct! {
    pub struct DDBracket {
        pub direct_declarator: Box<DirectDeclarator>,
        pub bracket: tokens::Bracket,
        pub qualifier_list: Option<TypeQualifierList>,
        pub assing_expr: Option<Expr>
    }
}

ast_struct! {
    pub struct DDParams {
        pub direct_declarator: Box<DirectDeclarator>,
        pub params: FnParams,
    }
}

ast_struct! {
    pub struct DDArgs{
        pub direct_declarator: Box<DirectDeclarator>,
        pub args: FnArgs
    }
}

use crate::{Parse, ParseStream, Result};

impl Parse for Declr {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            ty: parse.parse()?,
            vars: Punctuated::parse_non_terminated(parse)?,
            semi_colon: parse.parse()?,
        })
    }
}

impl Parse for DeclrList {
    fn parse(parse: ParseStream) -> Result<Self> {
        let mut items = vec![];
        while let Ok(declr) = parse.parse() {
            items.push(declr)
        }
        if items.is_empty() {
            Err(parse.error("expected at least one declaration"))
        } else {
            Ok(Self { items })
        }
    }
}

impl Parse for InitDeclarator {
    fn parse(parse: ParseStream) -> Result<Self> {
        let declarator = parse.parse()?;
        Ok(if parse.peek::<token![=]>() {
            Self::Init(InitDeclaratorInit {
                declarator,
                assign_tk: parse.parse()?,
                expr: parse.parse()?,
            })
        } else {
            Self::Uninit(InitDeclaratorUninit { declarator })
        })
    }
}

impl Parse for InitDeclaratorInit {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            declarator: parse.parse()?,
            assign_tk: parse.parse()?,
            expr: parse.parse()?,
        })
    }
}

impl Parse for InitDeclaratorUninit {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            declarator: parse.parse()?,
        })
    }
}

impl Parse for Declarator {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            ptr: parse.parse()?,
            direct_declarator: parse.parse()?,
        })
    }
}

impl Parse for DirectDeclarator {
    fn parse(parse: ParseStream) -> Result<Self> {
        let mut declr = if parse.peek::<Ident>() {
            DirectDeclarator::Ident(parse.parse()?)
        } else if parse.peek::<tokens::Paren>() {
            DirectDeclarator::Paren(parse.parse()?)
        } else {
            return Err(parse.error("expected parentheses or identifier"));
        };
        Ok({
            loop {
                if parse.peek::<tokens::Paren>() {
                    declr = Self::Params(DDParams {
                        direct_declarator: Box::new(declr),
                        params: parse.parse()?,
                    });
                } else if parse.peek::<tokens::Bracket>() {
                    let content;
                    declr = Self::Bracket(DDBracket {
                        direct_declarator: Box::new(declr),
                        bracket: bracketed!(content in parse)?,
                        qualifier_list: content.parse()?,
                        assing_expr: expr::parsing::parse_assign(&content).ok(),
                    });
                } else {
                    break declr;
                };
            }
        })
    }
}

impl Parse for DDParen {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in parse)?,
            declarator: content.parse()?,
        })
    }
}

impl Parse for DDParams {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            direct_declarator: parse.parse()?,
            params: parse.parse()?,
        })
    }
}

impl Parse for DDBracket {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            direct_declarator: parse.parse()?,
            bracket: bracketed!(content in parse)?,
            qualifier_list: content.parse()?,
            assing_expr: expr::parsing::parse_assign(&content).ok(),
        })
    }
}

impl Parse for DDArgs {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            direct_declarator: parse.parse()?,
            args: parse.parse()?,
        })
    }
}

mod quote {
    use super::*;
    use crate::{to_tokens, ToTokens, TokenStream};

    impl ToTokens for Declr {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                ty,
                vars,
                semi_colon,
            } = self;
            ty.to_tokens(tokens);
            vars.to_tokens(tokens);
            semi_colon.to_tokens(tokens);
        }
    }

    impl ToTokens for DeclrList {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            for item in &self.items {
                item.to_tokens(tokens)
            }
        }
    }

    impl ToTokens for InitDeclarator {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                InitDeclarator::Init(e) => e.to_tokens(tokens),
                InitDeclarator::Uninit(e) => e.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for InitDeclaratorInit {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                declarator,
                assign_tk,
                expr,
            } = self;
            declarator.to_tokens(tokens);
            assign_tk.to_tokens(tokens);
            expr.to_tokens(tokens);
        }
    }

    impl ToTokens for InitDeclaratorUninit {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { declarator } = self;
            declarator.to_tokens(tokens);
        }
    }

    impl ToTokens for Declarator {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                ptr,
                direct_declarator,
            } = self;
            ptr.to_tokens(tokens);
            direct_declarator.to_tokens(tokens)
        }
    }

    impl ToTokens for DirectDeclarator {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                DirectDeclarator::Ident(e) => e.to_tokens(tokens),
                DirectDeclarator::Paren(e) => e.to_tokens(tokens),
                DirectDeclarator::Bracket(e) => e.to_tokens(tokens),
                DirectDeclarator::Params(e) => e.to_tokens(tokens),
                DirectDeclarator::Args(e) => e.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for DDParen {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                paren: _,
                declarator,
            } = self;
            to_tokens::parenthesized(declarator).to_tokens(tokens);
        }
    }

    impl ToTokens for DDBracket {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                direct_declarator,
                bracket: _,
                qualifier_list,
                assing_expr,
            } = self;
            direct_declarator.to_tokens(tokens);
            to_tokens::bracketed(&to_tokens::multiple(|tokens| {
                qualifier_list.to_tokens(tokens);
                assing_expr.to_tokens(tokens);
            }))
            .to_tokens(tokens);
        }
    }

    impl ToTokens for DDParams {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                direct_declarator,
                params,
            } = self;
            direct_declarator.to_tokens(tokens);
            params.to_tokens(tokens);
        }
    }

    impl ToTokens for DDArgs {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                direct_declarator,
                args,
            } = self;
            direct_declarator.to_tokens(tokens);
            args.to_tokens(tokens);
        }
    }
}
