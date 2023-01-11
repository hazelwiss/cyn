use crate::declarations::Declr;
use crate::Parse;

pub struct File {
    pub declarations: Vec<Declr>,
}

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
        fn quote(&self) -> TokenStream {
            let mut ts = TokenStream::new_empty();
            for decl in &self.declarations {
                ts.extend(decl.quote())
            }
            ts
        }
    }
}
