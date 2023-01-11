ast_enum! {
    pub enum Lit{
        Int(LitInt),
        Str(LitStr),
    }
}

ast_struct! {
    pub struct LitInt{}
}

ast_struct! {
    pub struct LitStr{}
}
