use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Id)]
pub fn derive_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input as DeriveInput);
    let ident = derive.ident;

    quote! {
        impl aidee::Id for #ident {
            #[track_caller]
            fn from_index(index: usize) -> Self {
                Self(index.try_into().unwrap())
            }
            #[track_caller]
            fn index(self) -> usize {
                self.0.try_into().unwrap()
            }
        }
        impl ::core::clone::Clone for #ident {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::core::marker::Copy for #ident {}
    }
    .into()
}
