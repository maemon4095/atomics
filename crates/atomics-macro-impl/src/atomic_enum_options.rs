use proc_macro2::Span;
use syn::punctuated::Punctuated;

pub struct AtomicEnumOptions {
    pub is_flags: bool,
    pub is_ordered: bool,
}

impl Default for AtomicEnumOptions {
    fn default() -> Self {
        Self {
            is_flags: false,
            is_ordered: false,
        }
    }
}

impl syn::parse::Parse for AtomicEnumOptions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let options = Punctuated::<Option, syn::Token![,]>::parse_terminated(input)?;

        let mut is_flags = false;
        let mut is_ordered = false;

        for option in options.iter() {
            match option {
                Option::Flags(s) => {
                    if is_flags {
                        return Err(syn::Error::new(s.clone(), "flag was duplicated."));
                    }

                    is_flags = true;
                }
                Option::Ordered(s) => {
                    if is_ordered {
                        return Err(syn::Error::new(s.clone(), "ordered was duplicated."));
                    }
                    is_ordered = true;
                }
            }
        }

        Ok(Self {
            is_flags,
            is_ordered,
        })
    }
}

enum Option {
    Flags(Span),
    Ordered(Span),
}

impl syn::parse::Parse for Option {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) {
            let ident: syn::Ident = input.parse().unwrap();
            let str = ident.to_string();
            return match str.as_str() {
                "flags" => Ok(Option::Flags(ident.span())),
                "ordered" => Ok(Option::Ordered(ident.span())),
                _ => Err(syn::Error::new(
                    ident.span(),
                    "flags or ordered was expected",
                )),
            };
        }

        Err(syn::Error::new(
            input.span(),
            "flags or ordered was expected",
        ))
    }
}
