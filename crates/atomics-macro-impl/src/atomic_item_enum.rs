use syn::spanned::Spanned;

pub struct AtomicItemEnum {
    pub item: syn::ItemEnum,
}

impl syn::parse::Parse for AtomicItemEnum {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item: syn::ItemEnum = input.parse()?;

        if item.variants.iter().any(|e| !e.fields.is_empty()) {
            return Err(syn::Error::new(
                item.span(),
                "atomic enum variant must not have any field.",
            ));
        };

        Ok(Self { item })
    }
}

impl quote::ToTokens for AtomicItemEnum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.item.to_tokens(tokens);
    }
}
