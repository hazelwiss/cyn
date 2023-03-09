ast_struct! {
    pub struct InitializerList {}
}

ast_struct! {
    pub struct Initializer { }
}

use crate::{Parse, ParseStream, Result};

impl Parse for InitializerList {
    fn parse(parse: ParseStream) -> Result<Self> {
        todo!()
    }
}

impl Parse for Initializer {
    fn parse(parse: ParseStream) -> Result<Self> {
        todo!()
    }
}

mod quote {
    use super::*;
    use crate::{ToTokens, TokenStream};

    impl ToTokens for InitializerList {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            todo!()
        }
    }

    impl ToTokens for Initializer {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            todo!()
        }
    }
}
