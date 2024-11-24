use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use crate::generation_context::GenerationContext;

pub fn generate_impls(context: &GenerationContext) -> TokenStream {
    let basic_impl = generate_basic_atomic_impl(context);

    let flags_impl = if context.options.is_flags {
        Some(generate_flags_impl(context))
    } else {
        None
    };

    let ordered_impl = if context.options.is_ordered {
        Some(generate_ordered_impl(context))
    } else {
        None
    };

    quote! {
        #basic_impl
        #flags_impl
        #ordered_impl
    }
}

fn generate_basic_atomic_impl(context: &GenerationContext) -> TokenStream {
    let ty = &context.ty;
    let atomic_ty = &context.atomic_ty;
    let base_type = &context.base_ty;
    let ordering = &context.paths.ordering;
    let transmute = &context.paths.transmute;
    let result = &context.paths.result;
    let option = &context.paths.option;

    quote_spanned! { context.span =>
        #[allow(unused)]
        impl #atomic_ty {
            pub fn load(&self, order: #ordering) -> #ty {
                unsafe { #transmute(self.0.load(order)) }
            }

            pub fn store(&self, val: #ty, order: #ordering) {
                self.0.store(val as #base_type, order)
            }

            pub fn swap(&self, val: #ty, order: #ordering) -> #ty {
                unsafe { #transmute(self.0.swap(val as #base_type, order)) }
            }

            pub fn compare_exchange(
                &self,
                current: #ty,
                new: #ty,
                success: #ordering,
                failure: #ordering,
            ) -> #result<#ty, #ty> {
                unsafe {
                    self.0.compare_exchange(current as #base_type, new as #base_type, success, failure).map(|v| #transmute(v)).map_err(|v| #transmute(v))
                }
            }

            pub fn fetch_update<F>(
                &self,
                set_order: #ordering,
                fetch_order: #ordering,
                mut f: F,
            ) -> #result<#ty, #ty>
            where
                F: FnMut(#ty) -> #option<#ty> {
                unsafe {
                    self.0.fetch_update(set_order, fetch_order, |v| {
                        f(#transmute(v)).map(|v| v as #base_type)
                    })
                    .map(|v| #transmute(v))
                    .map_err(|v| #transmute(v))
                }
            }
        }
    }
}

fn generate_flags_impl(context: &GenerationContext) -> TokenStream {
    let ty = &context.ty;
    let atomic_ty = &context.atomic_ty;
    let base_ty = &context.base_ty;
    let ordering = &context.paths.ordering;
    let transmute = &context.paths.transmute;

    quote! {
        #[allow(unused)]
        impl #atomic_ty {
            pub fn fetch_and(&self, val: #ty, order: #ordering) -> #ty {
                unsafe { #transmute(self.0.fetch_and(val as #base_ty, order)) }
            }

            pub fn fetch_nand(&self, val: #ty, order: #ordering) -> #ty {
                unsafe { #transmute(self.0.fetch_nand(val as #base_ty, order)) }
            }

            pub fn fetch_or(&self, val: #ty, order: #ordering) -> #ty {
                unsafe { #transmute(self.0.fetch_or(val as #base_ty, order)) }
            }

            pub fn fetch_xor(&self, val: #ty, order: #ordering) -> #ty {
                unsafe { #transmute(self.0.fetch_xor(val as #base_ty, order)) }
            }
        }
    }
}

fn generate_ordered_impl(context: &GenerationContext) -> TokenStream {
    let ty = &context.ty;
    let atomic_ty = &context.atomic_ty;
    let base_ty = &context.base_ty;
    let ordering = &context.paths.ordering;
    let transmute = &context.paths.transmute;
    let option = &context.paths.option;

    quote! {
        impl ::std::cmp::Ord for #ty {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                unsafe {
                    #transmute::<_, &#base_ty>(self).cmp(#transmute::<_, &#base_ty>(other))
                }
            }
        }
        impl ::std::cmp::PartialEq for #ty {
            fn eq(&self, other: &Self) -> bool {
                unsafe {
                    #transmute::<_, &#base_ty>(self) == #transmute::<_, &#base_ty>(other)
                }
            }
        }

        impl ::std::cmp::Eq for #ty {  }

        impl ::std::cmp::PartialOrd for #ty {
            fn partial_cmp(&self, other: &Self) -> #option<::std::cmp::Ordering> {
                unsafe {
                    #transmute::<_, &#base_ty>(self).partial_cmp(#transmute::<_, &#base_ty>(other))
                }
            }
        }

        #[allow(unused)]
        impl #atomic_ty {
            pub fn fetch_min(&self, val: #ty, order: #ordering) -> #ty {
                unsafe { #transmute(self.0.fetch_min(val as #base_ty, order)) }
            }

            pub fn fetch_max(&self, val: #ty, order: #ordering) -> #ty {
                unsafe { #transmute(self.0.fetch_max(val as #base_ty, order)) }
            }
        }
    }
}
