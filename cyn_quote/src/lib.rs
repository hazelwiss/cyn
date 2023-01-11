

pub trait ToTokens {
    fn quote(&self) -> crate::buffers::TokenStream;
}
