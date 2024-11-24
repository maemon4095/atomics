macro_rules! declare_atomic_type {
    ($($type: ident),+) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub enum AtomicType {
            $($type(proc_macro2::Span)),*
        }

        impl syn::parse::Parse for AtomicType {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                mod keywords {
                    $(
                        syn::custom_keyword!($type);
                    )*
                }

                $(
                    if input.peek(keywords::$type) {
                        let kw = input.parse::<keywords::$type>().unwrap();
                        return Ok(Self::$type(<_ as syn::spanned::Spanned>::span(&kw)));
                    };
                )*

                Err(syn::Error::new(input.span(), concat!(declare_atomic_type!(@stringify $($type),*),  " was expected")))
            }
        }

        impl AtomicType {
            pub fn to_atomic(&self) -> syn::Ident {
                match self {
                    $(Self::$type(_) => {
                        atomic_ty(stringify!($type))
                    })*
                }
            }
        }

        impl quote::ToTokens for AtomicType {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                use quote::TokenStreamExt;
                match self {
                    $(Self::$type(span) => {
                        tokens.append(proc_macro2::Ident::new(stringify!($type), *span))
                    })*
                }
            }
        }
    };

    (@stringify $head:ty $(,$rest: ty)*) => {
        concat!(stringify!($head) $(,", ", stringify!($rest))*)
    }
}

fn atomic_ty(id: &str) -> syn::Ident {
    let mut chars = id.chars();
    let head = chars.next().unwrap();
    let rest = chars.as_str();
    let head = head.to_ascii_uppercase();

    quote::format_ident!("Atomic{}{}", head, rest)
}

declare_atomic_type!(u8, u16, u32, usize);
