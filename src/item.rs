use crate::{Declr, Fn, Parse, ParseStream, Result};

pub enum Item {
    Declr(Declr),
    Fn(Fn),
}

impl Parse for Item {
    fn parse(parse: ParseStream) -> Result<Self> {
        let fork = parse.fork();
        Ok(if let Ok(func) = fork.parse::<Fn>() {
            parse.set(fork);
            Self::Fn(func)
        } else {
            Self::Declr(parse.parse()?)
        })
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
