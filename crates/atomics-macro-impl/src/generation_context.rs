use syn::spanned::Spanned;

use crate::{
    atomic_enum_options::AtomicEnumOptions, atomic_item_enum::AtomicItemEnum,
    atomic_type::AtomicType,
};

pub struct GenerationContext {
    pub span: proc_macro2::Span,
    pub vis: syn::Visibility,
    pub ty: syn::Ident,
    pub atomic_ty: syn::Ident,
    pub base_ty: AtomicType,
    pub atomic_base_ty: syn::Path,
    pub options: AtomicEnumOptions,
    pub paths: Paths,
}

pub struct Paths {
    pub ordering: syn::Path,
    pub transmute: syn::Path,
    pub result: syn::Path,
    pub option: syn::Path,
}

impl Paths {
    fn new() -> Self {
        Self {
            ordering: syn::parse_quote!(::std::sync::atomic::Ordering),
            transmute: syn::parse_quote!(::std::mem::transmute),
            result: syn::parse_quote!(::std::result::Result),
            option: syn::parse_quote!(::std::option::Option),
        }
    }
}

impl GenerationContext {
    pub fn new(
        base_type: AtomicType,
        options: AtomicEnumOptions,
        enum_declaration: &AtomicItemEnum,
    ) -> GenerationContext {
        let vis = enum_declaration.item.vis.clone();
        let ty = enum_declaration.item.ident.clone();
        let atomic_base_ty: syn::Path = {
            let ty = base_type.to_atomic();
            syn::parse_quote!(::std::sync::atomic::#ty)
        };

        let atomic_ty = quote::format_ident!("Atomic{}", enum_declaration.item.ident);

        Self {
            span: enum_declaration.span(),
            vis,
            ty,
            atomic_ty,
            base_ty: base_type.clone(),
            atomic_base_ty,
            options,
            paths: Paths::new(),
        }
    }
}
