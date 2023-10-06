fn ty_inside_option(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != "Option" {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if !inner_ty.args.len() == 1 {
                return None;
            }
            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty {
                return Some(t);
            }
        }
    }
    None
}

fn ty_is_string(ty: &syn::Type) -> bool {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() == 1 && p.path.segments[0].ident == "String" {
            return true;
        }
    }
    false
}

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

    let tryfrom_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;

        if let Some(inner_ty) = ty_inside_option(ty) {
            if ty_is_string(inner_ty) {
                return quote! {
                    form.fields.#ident.value = value.#ident.clone();
                    #ident = value.#ident.clone();
                };
            } else {
                return quote! {
                    if let Some(ref v) = value.#ident {
                        form.fields.#ident.value = value.#ident.clone();
                        match v.parse::<#inner_ty>() {
                            Ok(v) => #ident = Some(v),
                            Err(e) => {
                                form.fields.#ident.errors.push(e.to_string());
                            }
                        }
                    } else {
                        form.fields.#ident.value = None;
                        #ident = None;
                    }
                };
            }
        }

        if ty_is_string(ty) {
            quote! {
                if let Some(ref v) = value.#ident {
                    form.fields.#ident.value = value.#ident.clone();
                    #ident = Some(v.clone());
                }
            }
        } else {
            quote! {
                if let Some(ref v) = value.#ident {
                    form.fields.#ident.value = value.#ident.clone();
                    match v.parse::<#ty>() {
                        Ok(v) => #ident = Some(v),
                        Err(e) => {
                            form.fields.#ident.errors.push(e.to_string());
                        }
                    }
                }
            }
        }
    });

    let none_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            let mut #ident = None;
        }
    });

    let unwrap_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        if let Some(_) = ty_inside_option(ty) {
            quote! {
                let #ident = #ident;
            }
        } else {
            quote! {
                let #ident = #ident.unwrap();
            }
        }
    });

    let field_idents = fields.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            #ident,
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

        impl TryFrom<&#unchecked_ident> for #ident {
            type Error = #html_form_ident;

            fn try_from(value: &#unchecked_ident) -> Result<Self, Self::Error> {
                #(#none_fields)*
                let mut form = #html_form_ident::new();

                #(#tryfrom_fields)*

                if form.has_errors() {
                    return Err(form);
                }

                #(#unwrap_fields)*

                Ok(#ident { #(#field_idents)* })
            }
        }

    };
    expanded.into()
}

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;
