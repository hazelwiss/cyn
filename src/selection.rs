use crate::{tokens, Expr, Stmnt};

ast_struct! {
    pub struct Switch {
        pub paren: tokens::Paren,
        pub selection: Box<Expr>,
        pub stmnt: Box<SwitchLabel>,
    }
}

ast_enum! {
    pub enum SwitchLabel {
        Case(Case),
        Default(Default)
    }
}

ast_struct! {
    pub struct Case {
        pub case_kw: token![case],
        pub const_expr: Box<Expr>,
        pub colon: token![:],
        pub stmnt: Box<Stmnt>,
    }
}

ast_struct! {
    pub struct Default {
        pub default_kw: token![default],
        pub colon: token![:],
        pub stmnt: Box<Stmnt>,
    }
}

ast_struct! {
    pub struct If {
        pub paren: tokens::Paren,
        pub condition: Box<Expr>,
        pub stmnt: Box<Stmnt>,
        pub else_stmnt: Option<Else>,
    }
}

ast_struct! {
    pub struct Else {
        pub else_kw: token![else],
        pub stmnt: Box<Stmnt>,
    }
}

use crate::{Parse, ParseStream, Result};

impl Parse for Switch {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in parse)?,
            selection: content.parse()?,
            stmnt: parse.parse()?,
        })
    }
}

impl Parse for SwitchLabel {
    fn parse(parse: ParseStream) -> Result<Self> {
        let lookahead = parse.lookahead1();
        Ok(if lookahead.peek::<token![case]>() {
            Self::Case(parse.parse()?)
        } else if lookahead.peek::<token![default]>() {
            Self::Default(parse.parse()?)
        } else {
            return Err(lookahead.error());
        })
    }
}

impl Parse for Case {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            case_kw: parse.parse()?,
            const_expr: parse.parse()?,
            colon: parse.parse()?,
            stmnt: parse.parse()?,
        })
    }
}

impl Parse for Default {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            default_kw: parse.parse()?,
            colon: parse.parse()?,
            stmnt: parse.parse()?,
        })
    }
}

impl Parse for If {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in parse)?,
            condition: content.parse()?,
            stmnt: parse.parse()?,
            else_stmnt: parse.parse()?,
        })
    }
}

impl Parse for Else {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(Self {
            else_kw: parse.parse()?,
            stmnt: parse.parse()?,
        })
    }
}

mod quote {
    use super::{Case, Default, Else, If, Switch, SwitchLabel};
    use crate::{to_tokens, ToTokens, TokenStream};

    impl ToTokens for Switch {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                paren: _,
                selection,
                stmnt,
            } = self;
            to_tokens::parenthesized(selection).to_tokens(tokens);
            stmnt.to_tokens(tokens);
        }
    }

    impl ToTokens for SwitchLabel {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                SwitchLabel::Case(s) => s.to_tokens(tokens),
                SwitchLabel::Default(s) => s.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for Case {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                case_kw,
                const_expr,
                colon,
                stmnt,
            } = self;
            case_kw.to_tokens(tokens);
            const_expr.to_tokens(tokens);
            colon.to_tokens(tokens);
            stmnt.to_tokens(tokens);
        }
    }

    impl ToTokens for Default {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                default_kw,
                colon,
                stmnt,
            } = self;
            default_kw.to_tokens(tokens);
            colon.to_tokens(tokens);
            stmnt.to_tokens(tokens);
        }
    }

    impl ToTokens for If {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                paren: _,
                condition,
                stmnt,
                else_stmnt,
            } = self;
            to_tokens::parenthesized(condition).to_tokens(tokens);
            stmnt.to_tokens(tokens);
            else_stmnt.to_tokens(tokens);
        }
    }

    impl ToTokens for Else {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { else_kw, stmnt } = self;
            else_kw.to_tokens(tokens);
            stmnt.to_tokens(tokens);
        }
    }
}
