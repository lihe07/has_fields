use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::token::Comma;

struct MacroInput {
    name: syn::Expr,
    fields: Punctuated<syn::LitStr, Comma>,
}

fn lit_str_parser(input: syn::parse::ParseStream) -> syn::Result<syn::LitStr> {
    let lit = input.parse::<syn::LitStr>()?;
    Ok(lit)
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        // Skip the comma
        input.parse::<Comma>()?;
        let fields = input.parse_terminated(lit_str_parser)?;
        Ok(MacroInput { name, fields })
    }
}

/// Check if given fields are Some(...) or not
///
/// If has missing fields, return a Err with the missing fields
///
/// If all fields are Some(...), return Ok(())
///
/// `require_fields!(&form, "field1", "field2", "...")`
#[proc_macro]
pub fn require_fields(item: TokenStream) -> TokenStream {
    // First: parse the input
    let MacroInput { name, fields } = parse_macro_input!(item as MacroInput);

    // Second: generate if-else statements
    let mut if_elses = quote!();
    for field in fields {
        let field_as_ident = syn::Ident::new(&field.value(), field.span());
        let if_else = quote! {
            if s.#field_as_ident.is_none() {
                missing_fields.push(#field);
            }
        };
        if_elses.extend(if_else);
    }

    quote! {
        {
            let s = #name;
            let mut missing_fields = Vec::new();

            #if_elses

            if missing_fields.is_empty() {
                Ok(())
            } else {
                Err(missing_fields)
            }
        }
    }
    .into()
}

/// Check if given fields are Some(...) or not
///
/// Gives a boolean result
///
/// `has_fields!(&form, "field1", "field2", "...")`
#[proc_macro]
pub fn has_fields(item: TokenStream) -> TokenStream {
    // First: parse the input
    let MacroInput { name, fields } = parse_macro_input!(item as MacroInput);

    // Second: generate if-else statements
    let mut if_elses = quote!();
    for field in fields {
        let field_as_ident = syn::Ident::new(&field.value(), field.span());
        let if_else = quote! {
            if s.#field_as_ident.is_none() {
                return false;
            }
        };
        if_elses.extend(if_else);
    }

    quote! {
        (|| {
            let s = #name;
            #if_elses
            true
        })()
    }
    .into()
}

#[proc_macro_derive(HasFields)]
pub fn derive_has_fields(item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    impl_has_fields(&ast)
}

fn impl_has_fields(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let mut fields = Vec::new();
    let mut non_option_fields = Vec::new();
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = &ast.data
    {
        for field in named {
            // if type of field is Option<T>, then we can use it
            if let syn::Type::Path(syn::TypePath { path, .. }) = &field.ty {
                if let Some(syn::PathSegment { ident, .. }) = path.segments.last() {
                    if ident == "Option" {
                        fields.push(field.ident.as_ref().unwrap());
                        continue;
                    }
                }
            }
            non_option_fields.push(field.ident.as_ref().unwrap());
        }
    }

    let mut if_elses = quote!();
    for field in fields {
        let if_else = quote! {
            if self.#field.is_some() {
                count += 1;
            }
        };
        if_elses.extend(if_else);
    }

    let non_option_fields = non_option_fields.len();

    quote! {
        impl #impl_generics HasFields for #name #ty_generics #where_clause {
            fn num_fields(&self) -> usize {
                let mut count = #non_option_fields;
                #if_elses
                count
            }
        }
    }
    .into()
}
