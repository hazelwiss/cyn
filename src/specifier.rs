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

ast_struct! {
    pub struct StorageClassList {
        pub items: Vec<StorageClass>,
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

macro_rules! impl_list {
    ($($ty:ty)*) => {
        $(
            impl Parse for $ty {
                fn parse(parse: ParseStream) -> Result<Self> {
                    let items = parse_into_vec(parse);
                    if items.is_empty() {
                        Err(parse.error("empty type qualifier list"))
                    } else {
                        Ok(Self { items })
                    }
                }
            }
        )*
    };
}

impl_list!(
    TypeQualifierList
    TypeSpecifierList
    FunctionSpecifierList
    StorageClassList
    AlignmentSpecifierList
);

ast_enum! {
    pub enum Specifier {
        Qualifier(TypeQualifier),
        Type(TypeSpecifier),
        Function(FunctionSpecifier),
        Alignment(AlignmentSpecifier),
        Storage(StorageClass),
    }
}

impl Parse for Specifier {
    fn parse(parse: ParseStream) -> Result<Self> {
        Ok(if parse.fork().parse::<FunctionSpecifier>().is_ok() {
            Self::Function(parse.parse()?)
        } else if parse.fork().parse::<AlignmentSpecifier>().is_ok() {
            Self::Alignment(parse.parse()?)
        } else if parse.fork().parse::<TypeSpecifier>().is_ok() {
            Self::Type(parse.parse()?)
        } else if parse.fork().parse::<TypeQualifier>().is_ok() {
            Self::Qualifier(parse.parse()?)
        } else if parse.fork().parse::<StorageClass>().is_ok() {
            Self::Storage(parse.parse()?)
        } else {
            return Err(parse.error("expected valid specifier"));
        })
    }
}

ast_struct! {
    pub struct SpecifierList {
        pub items: Vec<Specifier>
    }
}

impl Parse for SpecifierList {
    fn parse(parse: ParseStream) -> Result<Self> {
        let mut items = vec![];
        while let Ok(specifier) = parse.parse() {
            items.push(specifier)
        }
        if items.is_empty() {
            Err(parse.error("expected at least one specifier"))
        } else {
            Ok(Self { items })
        }
    }
}

mod quote {
    use super::*;
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

    impl ToTokens for StorageClass {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                StorageClass::Typedef(t) => t.to_tokens(tokens),
                StorageClass::Extern(t) => t.to_tokens(tokens),
                StorageClass::Static(t) => t.to_tokens(tokens),
                StorageClass::ThreadLocal(t) => t.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for AlignmentSpecifier {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
    }

    impl ToTokens for Specifier {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                Specifier::Qualifier(e) => e.to_tokens(tokens),
                Specifier::Type(e) => e.to_tokens(tokens),
                Specifier::Function(e) => e.to_tokens(tokens),
                Specifier::Alignment(e) => e.to_tokens(tokens),
                Specifier::Storage(e) => e.to_tokens(tokens),
            }
        }
    }

    macro_rules! impl_list {
        ($($ty:ty)*) => {
            $(
                impl ToTokens for $ty {
                    fn to_tokens(&self, tokens: &mut TokenStream) {
                        for item in &self.items {
                            item.to_tokens(tokens)
                        }
                    }
                }
            )*

        };
    }

    impl_list!(TypeQualifierList TypeSpecifierList FunctionSpecifierList StorageClassList AlignmentSpecifierList SpecifierList);
}
