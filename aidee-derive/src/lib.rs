use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(Id)]
pub fn derive_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input as DeriveInput);
    let ident = derive.ident;

    let field = match &derive.data {
        Data::Enum(..) | Data::Union(..) => panic!(),
        Data::Struct(strct) => &strct.fields.iter().next().unwrap().ty,
    };

    quote! {
        impl aidee::Id for #ident {
            #[track_caller]
            fn from_index(index: usize) -> Self {
                Self(<#field>::from_index(index))
            }
            #[track_caller]
            fn index(self) -> usize {
                self.0.index()
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl ::core::clone::Clone for #ident {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::core::marker::Copy for #ident {}
    }
    .into()
}
