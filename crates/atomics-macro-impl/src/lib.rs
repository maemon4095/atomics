mod atomic_enum_args;
mod atomic_enum_options;
mod atomic_item_enum;
mod atomic_type;
mod generate_impls;
mod generation_context;

use atomic_enum_args::AtomicEnumArgs;
use atomic_item_enum::AtomicItemEnum;
use generate_impls::generate_impls;
use generation_context::GenerationContext;
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;

pub fn atomic_enum(attr: TokenStream, body: TokenStream) -> TokenStream {
    let AtomicEnumArgs { base_type, options } = match syn::parse2(attr) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error(),
    };

    let enum_declaration: AtomicItemEnum = match syn::parse2(body) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error(),
    };

    let context = GenerationContext::new(base_type.clone(), options, &enum_declaration);
    let atomic_type_declaration = generate_atomic_type(&context);
    let impls = generate_impls(&context);

    quote_spanned! { enum_declaration.item.span() =>
        #[repr(#base_type)]
        #enum_declaration
        #atomic_type_declaration
        #impls
    }
}

fn generate_atomic_type(context: &GenerationContext) -> TokenStream {
    let vis = &context.vis;
    let ty = &context.ty;
    let base_type = &context.base_ty;
    let atomic_base_ty = &context.atomic_base_ty;
    let atomic_ty = &context.atomic_ty;
    let transmute = &context.paths.transmute;

    quote_spanned! { context.span =>
        #vis struct #atomic_ty(#atomic_base_ty);

        impl ::atomics::Atomicable for #ty {
            type Atomic = #atomic_ty;
        }

        impl ::std::convert::From<#ty> for #atomic_ty {
            fn from(value: #ty) -> Self {
                Self(#atomic_base_ty::from(value as #base_type))
            }
        }

        impl ::atomics::AtomicType for #atomic_ty {
            type Base = #ty;
            type Primitive = #atomic_base_ty;

            fn into_inner(self) -> Self::Base {
                unsafe {
                    #transmute(self.0.into_inner())
                }
            }
        }
    }
}
