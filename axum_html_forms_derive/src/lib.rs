#[proc_macro_derive(HtmlForm)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let ident = &ast.ident;
    let struct_vis = ast.vis;

    // <struct>Unchecked
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

    let all_option_strings = fields.iter().map(|f| {
        let ident = &f.ident;
        let vis = &f.vis;
        quote! {
            #vis #ident: std::option::Option<std::string::String>
        }
    });

    // <struct>Fields
    let fields_ident_string = format!("{}Fields", ident);
    let fields_ident = syn::Ident::new(&fields_ident_string, ident.span());

    let form_input_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        let vis = &f.vis;
        quote! {
            #vis #ident: axum_html_forms::FormInput
        }
    });

    // <struct>HtmlForm
    let html_form_ident_string = format!("{}HtmlForm", ident);
    let html_form_ident = syn::Ident::new(&html_form_ident_string, ident.span());

    let html_form_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            #ident: axum_html_forms::FormInput {
                // TODO we will need to get input_type from an attribute macro
                // or default to FormInputType::Text
                input_type: axum_html_forms::FormInputType::Text,
                name: String::from(stringify!(#ident)),
                label: String::from(stringify!(#ident)),
                value: None,
                errors: Vec::new(),
            }
        }
    });

    let empty_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            || !self.fields.#ident.errors.is_empty()
        }
    });

    let render_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            {
                self.fields.#ident.render()
            }
        }
    });

    // Final output
    let expanded = quote! {
        use axum_html_forms::HtmlField;
        use html_node::html;
        #[derive(Debug)]
        #struct_vis struct #unchecked_ident {
            #(#all_option_strings,)*
        }

        #[derive(Debug)]
        #struct_vis struct #fields_ident {
            #(#form_input_fields,)*
        }

        #[derive(Debug)]
        #struct_vis struct #html_form_ident {
            #struct_vis fields: #fields_ident,
            #struct_vis errors: axum_html_forms::FormErrors,
        }

        impl Default for #html_form_ident {
            fn default() -> Self {
                Self {
                    fields: #fields_ident {
                        #(#html_form_fields,)*
                    },
                    errors: Vec::new(),
                }
            }
        }

        impl #html_form_ident {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn has_errors(&self) -> bool {
                !self.errors.is_empty()
                #(#empty_fields)*
            }

            pub fn render(&self) -> html_node::Node {
                html_node::html! {
                    #(#render_fields)*
                }
            }
        }
    };
    expanded.into()
}

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;
