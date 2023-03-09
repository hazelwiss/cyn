use crate::{Block, Break, Continue, DoWhile, Expr, For, Goto, If, Label, Return, Switch, While};

ast_enum! {
    pub enum Stmnt {
        Label(Label),
        Block(Block),
        Expr(StmntExpr),
        Switch(Switch),
        If(If),
        While(While),
        DoWhile(DoWhile),
        For(For),
        Goto(Goto),
        Continue(Continue),
        Break(Break),
        Return(Return),
    }
}

ast_struct! {
    pub struct StmntExpr {
        pub expr: Box<Expr>,
        pub semi_colon: token![;],
    }
}

use crate::{tokens, Ident, Parse, ParseStream, Result};

impl Parse for Stmnt {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(if parse.peek::<token![if]>() {
            Self::If(parse.parse()?)
        } else if parse.peek::<token![switch]>() {
            Self::While(parse.parse()?)
        } else if parse.peek::<token![goto]>() {
            Self::Goto(parse.parse()?)
        } else if parse.peek::<token![continue]>() {
            Self::Continue(parse.parse()?)
        } else if parse.peek::<token![break]>() {
            Self::Break(parse.parse()?)
        } else if parse.peek::<token![return]>() {
            Self::Return(parse.parse()?)
        } else if parse.peek::<token![while]>() {
            Self::While(parse.parse()?)
        } else {
            if parse.peek::<tokens::Brace>() {
                Self::Block(parse.parse()?)
            } else if parse.peek::<Ident>() && parse.peek2::<token![:]>() {
                Self::Label(parse.parse()?)
            } else {
                Self::Expr(parse.parse()?)
            }
        })
    }
}

impl Parse for StmntExpr {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            expr: parse.parse()?,
            semi_colon: parse.parse()?,
        })
    }
}

mod quote {
    use super::{Stmnt, StmntExpr};
    use crate::{ToTokens, TokenStream};

    impl ToTokens for Stmnt {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                Stmnt::Label(s) => s.to_tokens(tokens),
                Stmnt::Block(s) => s.to_tokens(tokens),
                Stmnt::Expr(s) => s.to_tokens(tokens),
                Stmnt::Switch(s) => s.to_tokens(tokens),
                Stmnt::If(s) => s.to_tokens(tokens),
                Stmnt::While(s) => s.to_tokens(tokens),
                Stmnt::DoWhile(s) => s.to_tokens(tokens),
                Stmnt::For(s) => s.to_tokens(tokens),
                Stmnt::Goto(s) => s.to_tokens(tokens),
                Stmnt::Continue(s) => s.to_tokens(tokens),
                Stmnt::Break(s) => s.to_tokens(tokens),
                Stmnt::Return(s) => s.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for StmntExpr {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { expr, semi_colon } = self;
            expr.to_tokens(tokens);
            semi_colon.to_tokens(tokens);
        }
    }
}
