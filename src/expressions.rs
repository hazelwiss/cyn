ast_enum! {
    pub enum Expr{
        Ident(ExprIdent),
        Lit(ExprLit),
        Unary(ExprUnary),
    }
}

use crate::op::UnOp;
use crate::Ident;
use crate::Lit;

ast_struct! {
    pub struct ExprIdent {
        pub ident: Ident,
    }
}

ast_struct! {
    pub struct ExprLit {
        pub lit: Lit,
    }
}

ast_struct! {
    pub struct ExprUnary{
        pub op: UnOp,
        pub expr: Box<Expr>,
    }
}
