ast_struct! {
    pub struct Ty{}
}

use crate::Parse;

impl Parse for Ty {
    fn parse(parse: crate::ParseStream) -> crate::Result<Self> {
        Ok(Self {})
    }
}
