use crate::specifier::{
    AlignmentSpecifier, AlignmentSpecifierList, FunctionSpecifier, FunctionSpecifierList,
    TypeQualifier, TypeQualifierList, TypeSpecifier, TypeSpecifierList,
};

ast_struct! {
    pub struct Ty{
        pub function_list: FunctionSpecifierList,
        pub alignment_list: AlignmentSpecifierList,
        pub specifier_list: TypeSpecifierList,
        pub qualifier_list: TypeQualifierList,
        pub ptr: Option<Ptr>,
    }
}

ast_struct! {
    pub struct Ptr{
        pub asterix: token![*],
        pub qualifier_list: Option<TypeQualifierList>,
        pub next: Option<Box<Ptr>>
    }
}

use crate::{Parse, ParseStream, Result};

impl Parse for Ty {
    fn parse(parse: ParseStream) -> Result<Self> {
        let mut function_list = vec![];
        let mut alignment_list = vec![];
        let mut specifier_list = vec![];
        let mut qualifier_list = vec![];
        loop {
            if parse.fork().parse::<FunctionSpecifier>().is_ok() {
                function_list.push(parse.parse()?)
            } else if parse.fork().parse::<AlignmentSpecifier>().is_ok() {
                alignment_list.push(parse.parse()?)
            } else if parse.fork().parse::<TypeSpecifier>().is_ok() {
                specifier_list.push(parse.parse()?);
            } else if parse.fork().parse::<TypeQualifier>().is_ok() {
                qualifier_list.push(parse.parse()?)
            } else {
                break;
            }
        }
        if function_list.is_empty()
            && alignment_list.is_empty()
            && specifier_list.is_empty()
            && qualifier_list.is_empty()
        {
            Err(parse.error("expected type"))
        } else {
            Ok(Self {
                ptr: parse.parse()?,
                function_list: FunctionSpecifierList {
                    items: function_list,
                },
                alignment_list: AlignmentSpecifierList {
                    items: alignment_list,
                },
                specifier_list: TypeSpecifierList {
                    items: specifier_list,
                },
                qualifier_list: TypeQualifierList {
                    items: qualifier_list,
                },
            })
        }
    }
}

impl Parse for Ptr {
    fn parse(parse: ParseStream) -> Result<Self> {
        let asterix = parse.parse()?;
        let qualifier_list = parse.parse()?;
        Ok(if parse.peek::<token![*]>() {
            Self {
                asterix,
                qualifier_list,
                next: Some(parse.parse()?),
            }
        } else {
            Self {
                asterix,
                qualifier_list,
                next: None,
            }
        })
    }
}

mod quote {
    use super::{Ptr, Ty};
    use crate::{ToTokens, TokenStream};

    impl ToTokens for Ty {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                qualifier_list,
                specifier_list,
                function_list,
                alignment_list,
                ptr,
            } = self;
            alignment_list.to_tokens(tokens);
            function_list.to_tokens(tokens);
            qualifier_list.to_tokens(tokens);
            specifier_list.to_tokens(tokens);
            if let Some(ptr) = ptr {
                ptr.to_tokens(tokens);
            }
        }
    }

    impl ToTokens for Ptr {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                asterix,
                qualifier_list,
                next,
            } = self;
            asterix.to_tokens(tokens);
            qualifier_list.to_tokens(tokens);
            next.to_tokens(tokens);
        }
    }
}
