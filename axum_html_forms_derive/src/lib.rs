#[proc_macro_derive(HtmlForm)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // eprintln!("{:#?}", ast);

    let ident = &ast.ident;
    let struct_vis = ast.vis;
    let unchecked_ident_string = format!("{}Unchecked", ident);
    let unchecked_ident = syn::Ident::new(&unchecked_ident_string, ident.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let optionized_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        let vis = &f.vis;
        quote! {
            #vis #ident: std::option::Option<std::string::String>
        }
    });

    let expanded = quote! {
        #[derive(Debug)]
        #struct_vis struct #unchecked_ident {
            #(#optionized_fields,)*
        }

    };
    expanded.into()
}

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;
