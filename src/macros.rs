macro_rules! ast_struct {
    (pub struct $ident:ident $($rest:tt)*) => {
        pub struct $ident $($rest)*
    };
}

macro_rules! ast_enum {
    (pub enum $ident:ident $($rest:tt)*) => {
        pub enum $ident $($rest)*
    };
}
