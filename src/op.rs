ast_enum! {
    pub enum UnOp {
        PreInc(token![++]),
        PreDec(token![--]),
        Addr(token![&]),
        Deref(token![*]),
        Promote(token![+]),
        Neg(token![-]),
        Inv(token![~]),
        Not(token![!]),
    }
}

ast_enum! {
    pub enum BiOp {
        Mul(token![*]),
        Div(token![/]),
        Mod(token![%]),
        Add(token![+]),
        Sub(token![-]),
        LShft(token![<<]),
        RShft(token![>>]),
        Less(token![<]),
        Greater(token![>]),
        LessEq(token![<=]),
        GreaterEq(token![>=]),
        Eq(token![==]),
        NotEq(token![!=]),
        BwAnd(token![&]),
        BwXor(token![^]),
        BwOr(token![|]),
        And(token![&&]),
        Or(token![||]),
    }
}

ast_enum! {
    pub enum AssignOp {
        Eq(token![=]),
        Mul(token![*=]),
        Div(token![/=]),
        Mod(token![%=]),
        Add(token![+=]),
        Sub(token![-=]),
        LShft(token![<<=]),
        RShft(token![>>=]),
        And(token![&=]),
        Xor(token![^=]),
        Or(token![|=]),
    }
}

ast_enum! {
    pub enum PostOp {
        Inc(token![++]),
        Dec(token![--]),
    }
}

mod quote {
    use super::*;
    use crate::{ToTokens, TokenStream};

    impl ToTokens for UnOp {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                UnOp::PreInc(t) => t.to_tokens(tokens),
                UnOp::PreDec(t) => t.to_tokens(tokens),
                UnOp::Addr(t) => t.to_tokens(tokens),
                UnOp::Deref(t) => t.to_tokens(tokens),
                UnOp::Promote(t) => t.to_tokens(tokens),
                UnOp::Neg(t) => t.to_tokens(tokens),
                UnOp::Inv(t) => t.to_tokens(tokens),
                UnOp::Not(t) => t.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for BiOp {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                BiOp::Mul(t) => t.to_tokens(tokens),
                BiOp::Div(t) => t.to_tokens(tokens),
                BiOp::Mod(t) => t.to_tokens(tokens),
                BiOp::Add(t) => t.to_tokens(tokens),
                BiOp::Sub(t) => t.to_tokens(tokens),
                BiOp::LShft(t) => t.to_tokens(tokens),
                BiOp::RShft(t) => t.to_tokens(tokens),
                BiOp::Less(t) => t.to_tokens(tokens),
                BiOp::Greater(t) => t.to_tokens(tokens),
                BiOp::LessEq(t) => t.to_tokens(tokens),
                BiOp::GreaterEq(t) => t.to_tokens(tokens),
                BiOp::Eq(t) => t.to_tokens(tokens),
                BiOp::NotEq(t) => t.to_tokens(tokens),
                BiOp::BwAnd(t) => t.to_tokens(tokens),
                BiOp::BwXor(t) => t.to_tokens(tokens),
                BiOp::BwOr(t) => t.to_tokens(tokens),
                BiOp::And(t) => t.to_tokens(tokens),
                BiOp::Or(t) => t.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for AssignOp {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                AssignOp::Eq(t) => t.to_tokens(tokens),
                AssignOp::Mul(t) => t.to_tokens(tokens),
                AssignOp::Div(t) => t.to_tokens(tokens),
                AssignOp::Mod(t) => t.to_tokens(tokens),
                AssignOp::Add(t) => t.to_tokens(tokens),
                AssignOp::Sub(t) => t.to_tokens(tokens),
                AssignOp::LShft(t) => t.to_tokens(tokens),
                AssignOp::RShft(t) => t.to_tokens(tokens),
                AssignOp::And(t) => t.to_tokens(tokens),
                AssignOp::Xor(t) => t.to_tokens(tokens),
                AssignOp::Or(t) => t.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for PostOp {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                PostOp::Inc(t) => t.to_tokens(tokens),
                PostOp::Dec(t) => t.to_tokens(tokens),
            }
        }
    }
}
