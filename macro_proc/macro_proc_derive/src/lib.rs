use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(SayHi)]
pub fn say_hi_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let gene = quote! {
        impl SayHi for #name {
            fn say_hi() {
                println!(
                    "Hi Venomoooose {}", stringify!(#name)
                )
            }
        }
    };

    gene.into()
}
