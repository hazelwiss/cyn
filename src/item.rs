use crate::{tokens, Declr, Fn, Parse, ParseStream, Result, Ty};

pub enum Item {
    Declr(Declr),
    Fn(Fn),
}

impl Parse for Item {
    fn parse(parse: ParseStream) -> Result<Self> {
        let fork = parse.fork();
        fork.parse::<Option<Ty>>()?;
        loop {
            if fork.peek::<token![;]>() {
                break Ok(Self::Declr(parse.parse()?));
            } else if fork.peek::<tokens::Paren>() {
                break Ok(Self::Fn(parse.parse()?));
            } else if fork.is_empty() {
                break Err(parse.error("expected item"));
            } else {
                fork.skip();
            }
        }
    }
}

mod quote {
    use super::Item;
    use crate::{ToTokens, TokenStream};

    impl ToTokens for Item {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                Item::Declr(e) => e.to_tokens(tokens),
                Item::Fn(e) => e.to_tokens(tokens),
            }
        }
    }
}
