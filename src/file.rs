use crate::{Item, Parse, ParseStream};

pub struct File {
    pub declarations: Vec<Item>,
}

impl Parse for File {
    fn parse(parse: ParseStream) -> crate::Result<Self> {
        let mut declarations = vec![];
        while !parse.is_empty() {
            declarations.push(parse.parse()?)
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
