use crate::{Expr, Ident, Ptr, Punctuated, Ty};

ast_struct! {
    pub struct Declr {
        pub ty: Ty,
        pub vars: Punctuated<InitDeclarator, token![,]>,
        pub semi_colon: token![;],
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
    }
}

ast_struct! {
    pub struct DirectDeclaratorParen {}
}

ast_struct! {
    pub struct DirectDeclaratorBracket {}
}

ast_struct! {
    pub struct DirectDeclaratorParamList {}
}

ast_struct! {
    pub struct DirectDeclaratorIdentList {}
}

use crate::{Parse, ParseStream, Result};

impl Parse for Declr {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            ty: parse.parse()?,
            vars: Punctuated::parse_single_non_terminated(parse)?,
            semi_colon: parse.parse()?,
        })
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
        Ok(Self::Ident(parse.parse()?))
    }
}

mod quote {
    use super::*;
    use crate::{ToTokens, TokenStream};

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
            }
        }
    }
}
