use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn atomic_enum(attr: TokenStream, body: TokenStream) -> TokenStream {
    atomics_macro_impl::atomic_enum(attr.into(), body.into()).into()
}
