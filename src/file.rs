use crate::Item;

pub struct File {
    pub declarations: Vec<Item>,
}

use crate::Parse;

impl Parse for File {
    fn parse(parse: crate::ParseStream) -> crate::Result<Self> {
        let mut declarations = vec![];
        while !parse.is_empty() {
            declarations.push(parse.parse()?);
        }
        Ok(Self { declarations })
    }
}

mod quote {
    use super::File;
    use crate::{ToTokens, TokenStream};

    impl ToTokens for File {
        fn to_tokens(&self, extend: &mut TokenStream) {
            for decl in &self.declarations {
                decl.to_tokens(extend)
            }
        }
    }
}
