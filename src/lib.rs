use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Comma;

struct MacroInput {
    name: syn::Expr,
    fields: Punctuated<syn::LitStr, Comma>
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

/// Check if the given field is Some(...) or not
/// If has missing fields, return a Err with the missing fields
/// If all fields are Some(...), return Ok(())
/// `has_fields!(self, "name", "email", "password")`
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
    }.into()
}

