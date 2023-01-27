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
        Comma(ExprComma),
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
    pub struct ExprSizeof {
        pub sizeof: token![sizeof],
        pub expr: Box<Expr>,
    }
}

ast_struct! {
    pub struct ExprAlignof {
        pub alignof: token![_Alignof],
        pub paren: tokens::Paren,
        pub ty_name: Ident,
    }
}

ast_struct! {
    pub struct ExprComma {
        pub left: Box<Expr>,
        pub comma: token![,],
        pub right: Box<Expr>,
    }
}

mod parsing {
    use super::*;
    use crate::lit::Lit;
    use crate::op::{AssignOp, BiOp, PostOp, UnOp};
    use crate::parse::ParseStream;
    use crate::{tokens, Ident, Parse, Result};

    impl Parse for Expr {
        fn parse(parse: ParseStream) -> Result<Self> {
            parse_comma(parse)
        }
    }

    fn parse_primary(parse: ParseStream) -> Result<Expr> {
        Ok(if parse.peek::<Ident>() {
            Expr::Ident(ExprIdent {
                ident: parse.parse()?,
            })
        } else if parse.peek::<Lit>() {
            Expr::Lit(ExprLit {
                lit: parse.parse()?,
            })
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
                if parse.peek::<token![.]>() {
                    expr = Expr::Member(ExprMember {
                        body: Box::new(expr),
                        dot: parse.parse()?,
                        member: parse.parse()?,
                    })
                } else if parse.peek::<token![->]>() {
                    expr = Expr::MemberPtr(ExprMemberPtr {
                        body: Box::new(expr),
                        arrow: parse.parse()?,
                        member: parse.parse()?,
                    })
                } else if parse.peek::<token![++]>() {
                    expr = Expr::Postfix(ExprPostfix {
                        expr: Box::new(expr),
                        op: PostOp::Inc(parse.parse()?),
                    })
                } else if parse.peek::<token![--]>() {
                    expr = Expr::Postfix(ExprPostfix {
                        expr: Box::new(expr),
                        op: PostOp::Dec(parse.parse()?),
                    })
                } else if parse.peek::<tokens::Bracket>() {
                    let content;
                    expr = Expr::Subscript(ExprSubscript {
                        body: Box::new(expr),
                        bracket: bracketed!(content in &parse)?,
                        index: Box::new(content.parse()?),
                    })
                } else if parse.peek::<tokens::Paren>() {
                    expr = Expr::FnCall(ExprFnCall {
                        body: Box::new(expr),
                        args: parse.parse()?,
                    })
                } else {
                    break 'l;
                }
            }
            expr
        })
    }

    fn parse_unary(parse: ParseStream) -> Result<Expr> {
        Ok({
            let op = if parse.peek::<token![++]>() {
                UnOp::PreInc(parse.parse()?)
            } else if parse.peek::<token![--]>() {
                UnOp::PreDec(parse.parse()?)
            } else if parse.peek::<token![&]>() {
                UnOp::Addr(parse.parse()?)
            } else if parse.peek::<token![*]>() {
                UnOp::Deref(parse.parse()?)
            } else if parse.peek::<token![+]>() {
                UnOp::Promote(parse.parse()?)
            } else if parse.peek::<token![-]>() {
                UnOp::Neg(parse.parse()?)
            } else if parse.peek::<token![~]>() {
                UnOp::Inv(parse.parse()?)
            } else if parse.peek::<token![!]>() {
                UnOp::Not(parse.parse()?)
            } else if parse.peek::<token![sizeof]>() {
                return Ok(Expr::Sizeof(ExprSizeof {
                    sizeof: parse.parse()?,
                    expr: parse.parse()?,
                }));
            } else if parse.peek::<token![_Alignof]>() {
                let content;
                return Ok(Expr::Alignof(ExprAlignof {
                    alignof: parse.parse()?,
                    paren: parenthesized!(content in &parse)?,
                    ty_name: content.parse()?,
                }));
            } else {
                return parse_postfix(&parse);
            };
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
            op: if parse.peek::<token![*]>() {
                BiOp::Mul(parse.parse()?)
            } else if parse.peek::<token![/]>() {
                BiOp::Div(parse.parse()?)
            } else if parse.peek::<token![%]>() {
                BiOp::Mod(parse.parse()?)
            } else {
                return Ok(cast);
            },
            lhs: Box::new(cast),
            rhs: Box::new(parse_mul(parse)?),
        }))
    }

    fn parse_add(parse: ParseStream) -> Result<Expr> {
        let mul = parse_mul(parse)?;
        Ok(Expr::Binary(ExprBinary {
            op: if parse.peek::<token![+]>() {
                BiOp::Add(parse.parse()?)
            } else if parse.peek::<token![-]>() {
                BiOp::Sub(parse.parse()?)
            } else {
                return Ok(mul);
            },
            lhs: Box::new(mul),
            rhs: Box::new(parse_add(parse)?),
        }))
    }

    fn parse_bwshift(parse: ParseStream) -> Result<Expr> {
        let add = parse_add(parse)?;
        Ok(Expr::Binary(ExprBinary {
            op: if parse.peek::<token![<<]>() {
                BiOp::LShft(parse.parse()?)
            } else if parse.peek::<token![>>]>() {
                BiOp::RShft(parse.parse()?)
            } else {
                return Ok(add);
            },
            lhs: Box::new(add),
            rhs: Box::new(parse_bwshift(parse)?),
        }))
    }

    fn parse_relational(parse: ParseStream) -> Result<Expr> {
        let shift = parse_bwshift(parse)?;
        Ok(Expr::Binary(ExprBinary {
            op: if parse.peek::<token![<]>() {
                BiOp::Less(parse.parse()?)
            } else if parse.peek::<token![>]>() {
                BiOp::Greater(parse.parse()?)
            } else if parse.peek::<token![<=]>() {
                BiOp::LessEq(parse.parse()?)
            } else if parse.peek::<token![>=]>() {
                BiOp::GreaterEq(parse.parse()?)
            } else {
                return Ok(shift);
            },
            lhs: Box::new(shift),
            rhs: Box::new(parse_relational(parse)?),
        }))
    }

    fn parse_eq(parse: ParseStream) -> Result<Expr> {
        let relational = parse_relational(parse)?;
        Ok(Expr::Binary(ExprBinary {
            op: if parse.peek::<token![==]>() {
                BiOp::Eq(parse.parse()?)
            } else if parse.peek::<token![!=]>() {
                BiOp::NotEq(parse.parse()?)
            } else {
                return Ok(relational);
            },
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
        let fork = parse.fork();
        if let Ok(unary) = parse_unary(&fork) {
            let op = if fork.peek::<token![=]>() {
                AssignOp::Eq(fork.parse()?)
            } else if fork.peek::<token![*=]>() {
                AssignOp::Mul(fork.parse()?)
            } else if fork.peek::<token![/=]>() {
                AssignOp::Div(fork.parse()?)
            } else if fork.peek::<token![%=]>() {
                AssignOp::Mod(fork.parse()?)
            } else if fork.peek::<token![+=]>() {
                AssignOp::Add(fork.parse()?)
            } else if fork.peek::<token![-=]>() {
                AssignOp::Sub(fork.parse()?)
            } else if fork.peek::<token![<<=]>() {
                AssignOp::LShft(fork.parse()?)
            } else if fork.peek::<token![>>=]>() {
                AssignOp::RShft(fork.parse()?)
            } else if fork.peek::<token![&=]>() {
                AssignOp::And(fork.parse()?)
            } else if fork.peek::<token![^=]>() {
                AssignOp::Xor(fork.parse()?)
            } else if fork.peek::<token![|=]>() {
                AssignOp::Or(fork.parse()?)
            } else {
                return parse_cond(&parse);
            };
            let lhs = Box::new(unary);
            let rhs = Box::new(parse_assign(&fork)?);
            parse.set(fork);
            Ok(Expr::Assing(ExprAssign { lhs, op, rhs }))
        } else {
            parse_cond(parse)
        }
    }

    fn parse_comma(parse: ParseStream) -> Result<Expr> {
        let expr = parse_assign(parse)?;
        if parse.peek::<token![,]>() {
            Ok(Expr::Comma(ExprComma {
                left: Box::new(expr),
                comma: parse.parse()?,
                right: parse.parse()?,
            }))
        } else {
            Ok(expr)
        }
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
                Expr::Comma(e) => e.to_tokens(tokens),
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

    impl ToTokens for ExprComma {
        fn to_tokens(&self, tokens: &mut crate::TokenStream) {
            let Self { left, comma, right } = self;
            left.to_tokens(tokens);
            comma.to_tokens(tokens);
            right.to_tokens(tokens);
        }
    }
}
