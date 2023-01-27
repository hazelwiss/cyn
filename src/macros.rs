macro_rules! ast_struct {
    (pub struct $ident:ident $($rest:tt)*) => {
        pub struct $ident $($rest)*
    };
}

macro_rules! ast_enum {
    (pub enum $ident:ident $($rest:tt)*) => {
        pub enum $ident $($rest)*
    };
    (
        #[$($attr:meta),*]
        pub enum $ident:ident $($rest:tt)*
    ) => {
        #[$($attr),*]
        pub enum $ident $($rest)*
    };
}

macro_rules! ast_enum_simple {
    (pub enum $ident:ident {
        $($v:ident($t:ty)),* $(,)?
    }) => {
        ast_enum!(pub enum $ident { $( $v($t) ),* });

        impl  $crate::Parse for $ident{
            fn parse(parse: $crate::ParseStream) -> $crate::Result<Self> {
                let lookahead = parse.lookahead1();
                Ok($(
                    if lookahead.peek::<$t>(){
                        Self::$v(parse.parse()?)
                    }
                )else*
                else{
                    return Err(lookahead.error());
                })
            }
        }
    };
    (
        #[$($attr:meta),*]
        pub enum $ident:ident {
            $($v:ident($t:ty)),* $(,)?
        }
    ) => {

        ast_enum!(#[$($attr),*] pub enum $ident { $( $v($t) ),* });

        impl  $crate::Parse for $ident{
            fn parse(parse: $crate::ParseStream) -> $crate::Result<Self> {
                let lookahead = parse.lookahead1();
                Ok($(
                    if lookahead.peek::<$t>(){
                        Self::$v(parse.parse()?)
                    }
                )else*
                else{
                    return Err(lookahead.error());
                })
            }
        }
    };
}
