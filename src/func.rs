ast_struct! {
    pub struct FnArgs {}
}

ast_struct! {
    pub struct FnBody {}
}

use crate::{Parse, ParseStream, Result};

impl Parse for FnArgs {
    fn parse(parse: ParseStream) -> Result<Self> {
        todo!()
    }
}

impl Parse for FnBody {
    fn parse(parse: ParseStream) -> Result<Self> {
        todo!()
    }
}

mod quote {
    use super::*;
    use crate::{ToTokens, TokenStream};

    impl ToTokens for FnArgs {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            todo!()
        }
    }

    impl ToTokens for FnBody {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            todo!()
        }
    }
}
