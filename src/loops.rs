use crate::{tokens, Expr, Stmnt};

ast_struct! {
    pub struct While {
        pub while_kw: token![while],
        pub paren: tokens::Paren,
        pub condition: Box<Expr>,
        pub stmnt: Box<Stmnt>
    }
}

ast_struct! {
    pub struct DoWhile {
        pub do_kw: token![do],
        pub stmnt: Box<Stmnt>,
        pub while_kw: token![while],
        pub paren: tokens::Paren,
        pub condition: Box<Expr>,
        pub semi_colon: token![;],
    }
}

ast_struct! {
    pub struct For {
        pub for_kw: token![for],
        pub paren: tokens::Paren,
        pub initial: Option<Box<Expr>>,
        pub colon0: token![;],
        pub condition: Option<Box<Expr>>,
        pub colon1: token![;],
        pub post: Option<Box<Expr>>,
        pub stmnt: Box<Stmnt>,
    }
}

use crate::{Parse, ParseStream, Result};

impl Parse for While {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            while_kw: parse.parse()?,
            paren: parenthesized!(content in parse)?,
            condition: Box::new(content.parse()?),
            stmnt: parse.parse()?,
        })
    }
}

impl Parse for DoWhile {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            do_kw: parse.parse()?,
            stmnt: parse.parse()?,
            while_kw: parse.parse()?,
            paren: parenthesized!(content in parse)?,
            condition: content.parse()?,
            semi_colon: parse.parse()?,
        })
    }
}

impl Parse for For {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            for_kw: parse.parse()?,
            paren: parenthesized!(content in parse)?,
            initial: content.parse()?,
            colon0: content.parse()?,
            condition: content.parse()?,
            colon1: content.parse()?,
            post: content.parse()?,
            stmnt: parse.parse()?,
        })
    }
}

mod quote {
    use super::{DoWhile, For, While};
    use crate::{to_tokens, ToTokens, TokenStream};

    impl ToTokens for While {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                while_kw,
                paren: _,
                condition,
                stmnt,
            } = self;
            while_kw.to_tokens(tokens);
            to_tokens::parenthesized(condition).to_tokens(tokens);
            stmnt.to_tokens(tokens)
        }
    }

    impl ToTokens for DoWhile {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                do_kw,
                stmnt,
                while_kw,
                paren: _,
                condition,
                semi_colon,
            } = self;
            do_kw.to_tokens(tokens);
            stmnt.to_tokens(tokens);
            while_kw.to_tokens(tokens);
            to_tokens::parenthesized(condition).to_tokens(tokens);
            semi_colon.to_tokens(tokens);
        }
    }

    impl ToTokens for For {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                for_kw,
                paren: _,
                initial,
                colon0,
                condition,
                colon1,
                post,
                stmnt,
            } = self;
            for_kw.to_tokens(tokens);
            to_tokens::parenthesized(&to_tokens::multiple(|tokens| {
                initial.to_tokens(tokens);
                colon0.to_tokens(tokens);
                condition.to_tokens(tokens);
                colon1.to_tokens(tokens);
                post.to_tokens(tokens);
            }))
            .to_tokens(tokens);
            stmnt.to_tokens(tokens);
        }
    }
}
