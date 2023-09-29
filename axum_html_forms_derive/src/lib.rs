#[proc_macro_derive(HtmlForm)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    eprintln!("{:#?}", ast);
    let struct_visibility = ast.vis;
    let unchecked_name = format!("{}Unchecked", name);
    let unchecked_ident = syn::Ident::new(&unchecked_name, name.span());

    let expanded = quote! {
        #[derive(Debug)]
        #struct_visibility struct #unchecked_ident {
            pub foo: Option<String>,
            pub bar: Option<u8>,
        }

    };
    expanded.into()
}

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;
