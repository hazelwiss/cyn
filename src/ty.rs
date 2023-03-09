use crate::specifier::{SpecifierList, TypeQualifierList};

ast_struct! {
    pub struct Ty{
        pub specifiers: SpecifierList,
        pub ptr: Option<Ptr>,
    }
}

ast_struct! {
    pub struct Ptr{
        pub asterix: token![*],
        pub qualifier_list: Option<TypeQualifierList>,
        pub next: Option<Box<Ptr>>
    }
}

use crate::{Parse, ParseStream, Result};

impl Parse for Ty {
    fn parse(parse: ParseStream) -> Result<Self> {
        let specifiers = parse.parse::<SpecifierList>()?;
        if specifiers.items.is_empty() {
            Err(parse.error("empty specifier list"))
        } else {
            Ok(Self {
                ptr: parse.parse()?,
                specifiers,
            })
        }
    }
}

impl Parse for Ptr {
    fn parse(parse: ParseStream) -> Result<Self> {
        let asterix = parse.parse()?;
        let qualifier_list = parse.parse()?;
        Ok(if parse.peek::<token![*]>() {
            Self {
                asterix,
                qualifier_list,
                next: Some(parse.parse()?),
            }
        } else {
            Self {
                asterix,
                qualifier_list,
                next: None,
            }
        })
    }
}

mod quote {
    use super::{Ptr, Ty};
    use crate::{ToTokens, TokenStream};

    impl ToTokens for Ty {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self { specifiers, ptr } = self;
            specifiers.to_tokens(tokens);
            if let Some(ptr) = ptr {
                ptr.to_tokens(tokens);
            }
        }
    }

    impl ToTokens for Ptr {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                asterix,
                qualifier_list,
                next,
            } = self;
            asterix.to_tokens(tokens);
            qualifier_list.to_tokens(tokens);
            next.to_tokens(tokens);
        }
    }
}
