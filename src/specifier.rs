use crate::Ident;

ast_enum_simple! {
    pub enum TypeQualifier {
        Const(token![const]),
        Restrict(token![restrict]),
        Volatile(token![volatile]),
        Atomic(token![_Atomic]),
    }
}

ast_struct! {
    pub struct TypeQualifierList {
        pub items: Vec<TypeQualifier>,
    }
}

ast_enum_simple! {
    pub enum TypeSpecifier {
        Void(token![void]),
        Char(token![char]),
        Short(token![short]),
        Int(token![int]),
        Long(token![long]),
        Float(token![float]),
        Double(token![double]),
        Signed(token![signed]),
        Unsigned(token![unsigned]),
        Bool(token![_Bool]),
        Complex(token![_Complex]),
    }
}

ast_struct! {
    pub struct TypeSpecifierList {
        pub items: Vec<TypeSpecifier>,
    }
}

ast_enum_simple! {
    pub enum StorageClass {
        Typedef(token![typedef]),
        Extern(token![extern]),
        Static(token![static]),
        ThreadLocal(token![_Thread_local])
    }
}

ast_enum_simple! {
    pub enum FunctionSpecifier {
        Inline(token![inline]),
        Noreturn(token![_Noreturn]),
    }
}

ast_struct! {
    pub struct FunctionSpecifierList {
        pub items: Vec<FunctionSpecifier>,
    }
}

ast_enum! {
    pub enum AlignmentSpecifier {
    }
}

ast_struct! {
    pub struct AlignmentSpecifierList {
        pub items: Vec<AlignmentSpecifier>,
    }
}

use crate::{parse::parse_into_vec, Parse, ParseStream, Result};

impl Parse for AlignmentSpecifier {
    fn parse(parse: ParseStream) -> Result<Self> {
        Err(parse.error("unsupported alignment specifier"))
    }
}

impl Parse for TypeQualifierList {
    fn parse(parse: ParseStream) -> Result<Self> {
        let items = parse_into_vec(parse);
        if items.is_empty() {
            Err(parse.error("empty type qualifier list"))
        } else {
            Ok(Self { items })
        }
    }
}

impl Parse for TypeSpecifierList {
    fn parse(parse: ParseStream) -> Result<Self> {
        let items = parse_into_vec(parse);
        if items.is_empty() {
            Err(parse.error("empty type specifier list"))
        } else {
            Ok(Self { items })
        }
    }
}

impl Parse for FunctionSpecifierList {
    fn parse(parse: ParseStream) -> Result<Self> {
        let items = parse_into_vec(parse);
        if items.is_empty() {
            Err(parse.error("empty function specifier list"))
        } else {
            Ok(Self { items })
        }
    }
}

impl Parse for AlignmentSpecifierList {
    fn parse(parse: ParseStream) -> Result<Self> {
        let items = parse_into_vec(parse);
        if items.is_empty() {
            Err(parse.error("empty alignment specifier list"))
        } else {
            Ok(Self { items })
        }
    }
}

mod quote {
    use super::{
        AlignmentSpecifier, AlignmentSpecifierList, FunctionSpecifier, FunctionSpecifierList,
        TypeQualifier, TypeQualifierList, TypeSpecifier, TypeSpecifierList,
    };
    use crate::{ToTokens, TokenStream};

    impl ToTokens for TypeQualifier {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                TypeQualifier::Const(t) => t.to_tokens(tokens),
                TypeQualifier::Restrict(t) => t.to_tokens(tokens),
                TypeQualifier::Volatile(t) => t.to_tokens(tokens),
                TypeQualifier::Atomic(t) => t.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for TypeSpecifier {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                TypeSpecifier::Void(t) => t.to_tokens(tokens),
                TypeSpecifier::Char(t) => t.to_tokens(tokens),
                TypeSpecifier::Short(t) => t.to_tokens(tokens),
                TypeSpecifier::Int(t) => t.to_tokens(tokens),
                TypeSpecifier::Long(t) => t.to_tokens(tokens),
                TypeSpecifier::Float(t) => t.to_tokens(tokens),
                TypeSpecifier::Double(t) => t.to_tokens(tokens),
                TypeSpecifier::Signed(t) => t.to_tokens(tokens),
                TypeSpecifier::Unsigned(t) => t.to_tokens(tokens),
                TypeSpecifier::Bool(t) => t.to_tokens(tokens),
                TypeSpecifier::Complex(t) => t.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for FunctionSpecifier {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                FunctionSpecifier::Inline(t) => t.to_tokens(tokens),
                FunctionSpecifier::Noreturn(t) => t.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for AlignmentSpecifier {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
    }

    impl ToTokens for TypeQualifierList {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            for item in &self.items {
                item.to_tokens(tokens)
            }
        }
    }

    impl ToTokens for TypeSpecifierList {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            for item in &self.items {
                item.to_tokens(tokens)
            }
        }
    }

    impl ToTokens for FunctionSpecifierList {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            for item in &self.items {
                item.to_tokens(tokens)
            }
        }
    }

    impl ToTokens for AlignmentSpecifierList {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            for item in &self.items {
                item.to_tokens(tokens)
            }
        }
    }
}
