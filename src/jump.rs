use crate::{Expr, Ident};

ast_struct! {
    pub struct Goto {
        pub goto_kw: token![goto],
        pub ident: Ident,
        pub semi_colon: token![;],
    }
}

ast_struct! {
    pub struct Continue {
        pub continue_kw: token![continue],
        pub semi_colon: token![;],
    }
}

ast_struct! {
    pub struct Break {
        pub break_kw: token![break],
        pub semi_colon: token![;],
    }
}

ast_struct! {
    pub struct Return {
        pub return_kw: token![return],
        pub expr: Box<Expr>,
        pub semi_colon: token![;],
    }
}

use crate::{Parse, ParseStream, Result};

impl Parse for Goto {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            goto_kw: parse.parse()?,
            ident: parse.parse()?,
            semi_colon: parse.parse()?,
        })
    }
}

impl Parse for Continue {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            continue_kw: parse.parse()?,
            semi_colon: parse.parse()?,
        })
    }
}

impl Parse for Break {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            break_kw: parse.parse()?,
            semi_colon: parse.parse()?,
        })
    }
}

impl Parse for Return {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            return_kw: parse.parse()?,
            expr: parse.parse()?,
            semi_colon: parse.parse()?,
        })
    }
}

mod quote {
    use super::{Break, Continue, Goto, Return};
    use crate::{ToTokens, TokenStream};

    impl ToTokens for Goto {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                goto_kw,
                ident,
                semi_colon,
            } = self;
            goto_kw.to_tokens(tokens);
            ident.to_tokens(tokens);
            semi_colon.to_tokens(tokens);
        }
    }

    impl ToTokens for Continue {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                continue_kw,
                semi_colon,
            } = self;
            continue_kw.to_tokens(tokens);
            semi_colon.to_tokens(tokens);
        }
    }

    impl ToTokens for Break {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                break_kw,
                semi_colon,
            } = self;
            break_kw.to_tokens(tokens);
            semi_colon.to_tokens(tokens);
        }
    }

    impl ToTokens for Return {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                return_kw,
                expr,
                semi_colon,
            } = self;
            return_kw.to_tokens(tokens);
            expr.to_tokens(tokens);
            semi_colon.to_tokens(tokens);
        }
    }
}
