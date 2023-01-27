use crate::{tokens, Ty};

ast_struct! {
    pub struct Block {
        pub brace: tokens::Brace,
        pub items: Vec<BlockItem>,
    }
}

use crate::{Declr, Stmnt};

ast_enum! {
    pub enum BlockItem {
        Stmnt(Stmnt),
        Declr(Declr),
    }
}

use crate::{Parse, ParseStream, Result};

impl Parse for Block {
    fn parse(parse: ParseStream) -> Result<Self> {
        let content;
        let brace = braced!(content in parse)?;
        let mut items = vec![];
        while !content.is_empty() {
            items.push(content.parse()?)
        }
        Ok(Self { brace, items })
    }
}

impl Parse for BlockItem {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(if parse.fork().parse::<Ty>().is_ok() {
            Self::Declr(parse.parse()?)
        } else {
            Self::Stmnt(parse.parse()?)
        })
    }
}

mod quote {
    use super::{Block, BlockItem};
    use crate::{to_tokens, ToTokens, TokenStream};

    impl ToTokens for Block {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { brace: _, items } = self;
            to_tokens::braced(items).to_tokens(tokens);
        }
    }

    impl ToTokens for BlockItem {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                BlockItem::Stmnt(stmnt) => stmnt.to_tokens(tokens),
                BlockItem::Declr(declr) => declr.to_tokens(tokens),
            }
        }
    }
}
