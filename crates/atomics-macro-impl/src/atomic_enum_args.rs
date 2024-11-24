use crate::{atomic_enum_options::AtomicEnumOptions, atomic_type::AtomicType};

pub struct AtomicEnumArgs {
    pub base_type: AtomicType,
    pub options: AtomicEnumOptions,
}

impl syn::parse::Parse for AtomicEnumArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let base_type: AtomicType = input.parse()?;
        let options = if input.peek(syn::Token![;]) {
            input.parse::<syn::Token![;]>().unwrap();
            AtomicEnumOptions::parse(input)?
        } else {
            AtomicEnumOptions::default()
        };

        Ok(Self { base_type, options })
    }
}
