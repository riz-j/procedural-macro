extern crate proc_macro;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Reflective)]
pub fn reflective_derive_macro(token: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(token).unwrap();
    let ident = ast.ident;
    let ident_str = ident.to_string();

    quote::quote! {
        impl Reflective for #ident {
            fn name(&self) -> &'static str {
                #ident_str
            }
        }
    }
    .into()
}
