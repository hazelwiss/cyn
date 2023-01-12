ast_enum! {
    pub enum Expr{
        Ident(ExprIdent),
        Lit(ExprLit),
        Unary(ExprUnary),
        Binary(ExprBinary),
        Paren(ExprParen),
        Assing(ExprAssign),
        Subscript(ExprSubscript),
        FnCall(ExprFnCall),
        Member(ExprMember),
        MemberPtr(ExprMemberPtr),
        Postfix(ExprPostfix),
        Sizeof(ExprSizeof),
        Alignof(ExprAlignof),
    }
}

use crate::func::FnArgs;
use crate::op::{AssignOp, PostOp};
use crate::op::{BiOp, UnOp};
use crate::tokens;
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
    pub struct ExprUnary {
        pub op: UnOp,
        pub expr: Box<Expr>,
    }
}

ast_struct! {
    pub struct ExprBinary{
        pub lhs: Box<Expr>,
        pub op: BiOp,
        pub rhs: Box<Expr>,
    }
}

ast_struct! {
    pub struct ExprParen {
        pub paren: tokens::Paren,
        pub expr: Box<Expr>,
    }
}

ast_struct! {
    pub struct ExprAssign {
        pub lhs: Box<Expr>,
        pub op: AssignOp,
        pub rhs: Box<Expr>,
    }
}

ast_struct! {
    pub struct ExprSubscript {
        pub body: Box<Expr>,
        pub bracket: tokens::Bracket,
        pub index: Box<Expr>,
    }
}

ast_struct! {
    pub struct ExprFnCall {
        pub body: Box<Expr>,
        pub args: FnArgs,
    }
}

ast_struct! {
    pub struct ExprMember {
        pub body: Box<Expr>,
        pub dot: token![.],
        pub member: Ident,
    }
}

ast_struct! {
    pub struct ExprMemberPtr {
        pub body: Box<Expr>,
        pub arrow: token![->],
        pub member: Ident,
    }
}

ast_struct! {
    pub struct ExprPostfix {
        pub expr: Box<Expr>,
        pub op: PostOp,
    }
}

ast_struct! {
    pub struct ExprSizeof{
        pub sizeof: token![sizeof],
        pub expr: Box<Expr>,
    }
}

ast_struct! {
    pub struct ExprAlignof{
        pub alignof: token![_Alignof],
        pub paren: tokens::Paren,
        pub ty_name: Ident,
    }
}

use crate::Parse;
use crate::ParseStream;
use crate::Result;

impl Parse for Expr {
    fn parse(parse: ParseStream) -> Result<Self> {
        parsing::parse_expr(parse)
    }
}

mod parsing {
    use super::*;
    use crate::lit::Lit;
    use crate::op::{AssignOp, UnOp};
    use crate::parse::{ParseStream, Result};
    use crate::tokens;

    pub fn parse_expr(parse: ParseStream) -> Result<Expr> {
        parse_comma(parse)
    }

    fn parse_primary(parse: ParseStream) -> Result<Expr> {
        Ok(if let Ok(ident) = parse.parse::<Ident>() {
            Expr::Ident(ExprIdent { ident })
        } else if let Ok(lit) = parse.parse::<Lit>() {
            Expr::Lit(ExprLit { lit })
        } else if parse.peek::<tokens::Paren>() {
            let content;
            let paren = parenthesized!(content in parse)?;
            let expr = Box::new(content.parse()?);
            Expr::Paren(ExprParen { paren, expr })
        } else {
            return Err(parse.error("expected primary expression."));
        })
    }

    fn parse_postfix(parse: ParseStream) -> Result<Expr> {
        Ok(if false {
            //let content;
            //let _paren = parenthesized!(content in parse)?;
            //let ty = content.parse()?;
            //let content;
            //let _brace = braced!(content in parse)?;
            //let initializer_list = Punctuated::parse_terminated(&content)?;
            //Expr::Construct(Construct {
            //    ty,
            //    initializer_list,
            //})
            unimplemented!()
        } else {
            let mut expr = parse_primary(parse)?;
            'l: loop {
                match_tokens!(
                    parse;
                    token![.] => {
                        expr = Expr::Member(ExprMember{
                            body: Box::new(expr),
                            dot: parse.parse()?,
                            member: parse.parse()?,
                        })
                    },
                    token![->] => {
                        expr = Expr::MemberPtr(ExprMemberPtr{
                            body: Box::new(expr),
                            arrow: parse.parse()?,
                            member: parse.parse()?,
                        })
                    },
                    token![++] => {
                        expr = Expr::Postfix(ExprPostfix {
                            expr: Box::new(expr),
                            op: PostOp::Inc(parse.parse()?),
                        })
                    },
                    token![--] => {
                        expr = Expr::Postfix(ExprPostfix {
                            expr: Box::new(expr),
                            op: PostOp::Dec(parse.parse()?),
                        })
                    },
                    tokens::Bracket => {
                        let content;
                        expr = Expr::Subscript(ExprSubscript{ body: Box::new(expr), bracket: bracketed!(content in &parse)?, index: Box::new(content.parse()?) })
                    },
                    tokens::Paren => {
                        expr = Expr::FnCall(ExprFnCall{ body: Box::new(expr), args: parse.parse()? })
                    },
                    ; break 'l,
                )
            }
            expr
        })
    }

    fn parse_unary(parse: ParseStream) -> Result<Expr> {
        Ok({
            let op = match_tokens!(
                parse;
                token![++] => UnOp::PreInc(parse.parse()?),
                token![--] => UnOp::PreDec(parse.parse()?),
                token![&] => UnOp::Addr(parse.parse()?),
                token![*] => UnOp::Deref(parse.parse()?),
                token![+] => UnOp::Promote(parse.parse()?),
                token![-] => UnOp::Neg(parse.parse()?),
                token![~] => UnOp::Inv(parse.parse()?),
                token![!] => UnOp::Not(parse.parse()?),
                token![sizeof] => return Ok(Expr::Sizeof(ExprSizeof {
                    sizeof: parse.parse()?,
                    expr: Box::new(parse.parse()?)
                })),
                token![_Alignof] => {
                    let content;
                    return Ok(Expr::Alignof(ExprAlignof{
                        alignof: parse.parse()?,
                        paren: parenthesized!(content in &parse)?,
                        ty_name: content.parse()?
                    }))
                },
                ; return parse_postfix(&parse)
            );
            Expr::Unary(ExprUnary {
                op: op,
                expr: Box::new(parse_cast(parse)?),
            })
        })
    }

    fn parse_cast(parse: ParseStream) -> Result<Expr> {
        if false {
            //let content;
            //let paren = parenthesized!(content in parse)?;
            //let ty = content.parse()?;
            //let expr = Box::new(parse.parse()?);
            //Ok(Expr::Cast(Cast { paren, ty, expr }))
            todo!()
        } else {
            parse_unary(parse)
        }
    }

    fn parse_mul(parse: ParseStream) -> Result<Expr> {
        let cast = parse_cast(parse)?;
        Ok(Expr::Binary(ExprBinary {
            op: match_tokens!(
                parse;
                token![*] => BiOp::Mul(parse.parse()?),
                token![/] => BiOp::Div(parse.parse()?),
                token![%] => BiOp::Mod(parse.parse()?),
                ; return Ok(cast)
            ),
            lhs: Box::new(cast),
            rhs: Box::new(parse_mul(parse)?),
        }))
    }

    fn parse_add(parse: ParseStream) -> Result<Expr> {
        let mul = parse_mul(parse)?;
        Ok(Expr::Binary(ExprBinary {
            op: match_tokens!(
                parse;
                token![+] => BiOp::Add(parse.parse()?),
                token![-] => BiOp::Sub(parse.parse()?),
                ; return Ok(mul)
            ),
            lhs: Box::new(mul),
            rhs: Box::new(parse_add(parse)?),
        }))
    }

    fn parse_bwshift(parse: ParseStream) -> Result<Expr> {
        let add = parse_add(parse)?;
        Ok(Expr::Binary(ExprBinary {
            op: match_tokens!(
                parse;
                token![<<] => BiOp::LShft(parse.parse()?),
                token![>>] => BiOp::RShft(parse.parse()?),
                ; return Ok(add)
            ),
            lhs: Box::new(add),
            rhs: Box::new(parse_bwshift(parse)?),
        }))
    }

    fn parse_relational(parse: ParseStream) -> Result<Expr> {
        let shift = parse_bwshift(parse)?;
        Ok(Expr::Binary(ExprBinary {
            op: match_tokens!(
                parse;
                token![<] => BiOp::Less(parse.parse()?),
                token![>] => BiOp::Greater(parse.parse()?),
                token![<=] => BiOp::LessEq(parse.parse()?),
                token![>=] => BiOp::GreaterEq(parse.parse()?),
                ; return Ok(shift)
            ),
            lhs: Box::new(shift),
            rhs: Box::new(parse_relational(parse)?),
        }))
    }

    fn parse_eq(parse: ParseStream) -> Result<Expr> {
        let relational = parse_relational(parse)?;
        Ok(Expr::Binary(ExprBinary {
            op: match_tokens!(
                parse;
                token![==] => BiOp::Eq(parse.parse()?),
                token![!=] => BiOp::NotEq(parse.parse()?),
                ; return Ok(relational)
            ),
            lhs: Box::new(relational),
            rhs: Box::new(parse_eq(parse)?),
        }))
    }

    fn parse_bwand(parse: ParseStream) -> Result<Expr> {
        let eq = parse_eq(parse)?;
        Ok(if parse.peek::<token![&]>() {
            Expr::Binary(ExprBinary {
                lhs: Box::new(eq),
                op: BiOp::BwAnd(parse.parse()?),
                rhs: Box::new(parse_bwand(parse)?),
            })
        } else {
            eq
        })
    }

    fn parse_bwxor(parse: ParseStream) -> Result<Expr> {
        let and = parse_bwand(parse)?;
        Ok(if parse.peek::<token![^]>() {
            Expr::Binary(ExprBinary {
                lhs: Box::new(and),
                op: BiOp::BwOr(parse.parse()?),
                rhs: Box::new(parse_bwxor(parse)?),
            })
        } else {
            and
        })
    }

    fn parse_bwor(parse: ParseStream) -> Result<Expr> {
        let xor = parse_bwxor(parse)?;
        Ok(if parse.peek::<token![|]>() {
            Expr::Binary(ExprBinary {
                lhs: Box::new(xor),
                op: BiOp::BwOr(parse.parse()?),
                rhs: Box::new(parse_bwor(parse)?),
            })
        } else {
            xor
        })
    }

    fn parse_land(parse: ParseStream) -> Result<Expr> {
        let or = parse_bwor(parse)?;
        Ok(if parse.peek::<token![&&]>() {
            Expr::Binary(ExprBinary {
                lhs: Box::new(or),
                op: BiOp::And(parse.parse()?),
                rhs: Box::new(parse_land(parse)?),
            })
        } else {
            or
        })
    }

    fn parse_lor(parse: ParseStream) -> Result<Expr> {
        let and = parse_land(parse)?;
        Ok(if parse.peek::<token![||]>() {
            Expr::Binary(ExprBinary {
                lhs: Box::new(and),
                op: BiOp::Or(parse.parse()?),
                rhs: Box::new(parse_lor(parse)?),
            })
        } else {
            and
        })
    }

    fn parse_cond(parse: ParseStream) -> Result<Expr> {
        let cond_expr = parse_lor(parse)?;
        Ok(if parse.peek::<token![?]>() {
            //let true_expr = Box::new(parse.parse()?);
            //let colon = parse.parse::<token![:]>()?;
            //let false_expr = Box::new(parse_cond(parse)?);
            //Expr::Ternary(Ternary {
            //    cond_expr: Box::new(cond_expr),
            //    true_expr,
            //    false_expr,
            //})
            todo!()
        } else {
            cond_expr
        })
    }

    fn parse_assign(parse: ParseStream) -> Result<Expr> {
        let clone = parse.fork();
        if let Ok(unary) = parse_unary(&clone) {
            let lhs = Box::new(unary);
            let op = match_tokens!(
                clone;
                token![=] => AssignOp::Eq(parse.parse()?),
                token![*=] => AssignOp::Mul(parse.parse()?),
                token![/=] => AssignOp::Div(parse.parse()?),
                token![%=] => AssignOp::Mod(parse.parse()?),
                token![+=] => AssignOp::Add(parse.parse()?),
                token![-=] => AssignOp::Sub(parse.parse()?),
                token![<<=] => AssignOp::LShft(parse.parse()?),
                token![>>=] => AssignOp::RShft(parse.parse()?),
                token![&=] => AssignOp::And(parse.parse()?),
                token![^=] => AssignOp::Xor(parse.parse()?),
                token![|=] => AssignOp::Or(parse.parse()?),
                ; return parse_cond(&parse)
            );
            let rhs = Box::new(parse_assign(&clone)?);
            parse.update_cursor(clone.cursor());
            Ok(Expr::Assing(ExprAssign { lhs, op, rhs }))
        } else {
            parse_cond(parse)
        }
    }

    fn parse_comma(parse: ParseStream) -> Result<Expr> {
        let assingment_expr = parse_assign(parse)?;
        Ok(if parse.peek::<token![,]>() {
            //Expr::Comma(Comma {
            //    l: Box::new(assingment_expr),
            //    r: Box::new(parse_comma(parse)?),
            //})
            todo!()
        } else {
            assingment_expr
        })
    }
}

mod quote {
    use super::*;
    use crate::{to_tokens, ToTokens};

    impl ToTokens for Expr {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            match self {
                Expr::Ident(e) => e.to_tokens(tokens),
                Expr::Lit(e) => e.to_tokens(tokens),
                Expr::Unary(e) => e.to_tokens(tokens),
                Expr::Binary(e) => e.to_tokens(tokens),
                Expr::Paren(e) => e.to_tokens(tokens),
                Expr::Assing(e) => e.to_tokens(tokens),
                Expr::Subscript(e) => e.to_tokens(tokens),
                Expr::FnCall(e) => e.to_tokens(tokens),
                Expr::Member(e) => e.to_tokens(tokens),
                Expr::MemberPtr(e) => e.to_tokens(tokens),
                Expr::Postfix(e) => e.to_tokens(tokens),
                Expr::Sizeof(e) => e.to_tokens(tokens),
                Expr::Alignof(e) => e.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for ExprIdent {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self { ident } = self;
            ident.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprLit {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self { lit } = self;
            lit.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprUnary {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self { op, expr } = self;
            op.to_tokens(tokens);
            expr.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprBinary {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self { lhs, op, rhs } = self;
            lhs.to_tokens(tokens);
            op.to_tokens(tokens);
            rhs.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprParen {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self { paren: _, expr } = self;
            to_tokens::parenthesized(expr.as_ref()).to_tokens(tokens)
        }
    }

    impl ToTokens for ExprAssign {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self { lhs, op, rhs } = self;
            lhs.to_tokens(tokens);
            op.to_tokens(tokens);
            rhs.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprSubscript {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self {
                body,
                bracket: _,
                index,
            } = self;
            body.to_tokens(tokens);
            to_tokens::bracketed(index.as_ref()).to_tokens(tokens)
        }
    }

    impl ToTokens for ExprFnCall {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self { body, args } = self;
            body.to_tokens(tokens);
            args.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprMember {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self { body, dot, member } = self;
            body.to_tokens(tokens);
            dot.to_tokens(tokens);
            member.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprMemberPtr {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self {
                body,
                arrow,
                member,
            } = self;
            body.to_tokens(tokens);
            arrow.to_tokens(tokens);
            member.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprPostfix {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self { expr, op } = self;
            expr.to_tokens(tokens);
            op.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprSizeof {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self { sizeof, expr } = self;
            sizeof.to_tokens(tokens);
            expr.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprAlignof {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self {
                alignof,
                paren: _,
                ty_name,
            } = self;
            alignof.to_tokens(tokens);
            to_tokens::parenthesized(ty_name).to_tokens(tokens);
        }
    }
}
