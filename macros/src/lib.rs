extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro::TokenTree;
use quote::quote;
use syn::{parse, parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn init_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_value: String;
    if attr.clone().into_iter().count() != 1 {
        return parse::Error::new(Span::call_site(), 
            "This property only accepts one parameter").to_compile_error().into();
    }
    if let Some(TokenTree::Literal(lit)) = attr.into_iter().next() {
        attr_value = lit.to_string().trim_matches('"').to_string();
    }
    else {
        return parse::Error::new(Span::call_site(), 
            "Only accept parameters of type Literal").to_compile_error().into();
    }
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let static_ident: Ident = syn::parse_str(format!("__rt_init_{}", fn_name).as_str())
                            .expect("Failed to parse Ident from string");
    let link_section_name = format!(".rti_fn.{}", attr_value);
    let expanded = quote! {
        #[used]
        #[link_section = #link_section_name]
        static #static_ident: fn() = #fn_name;
        #input
    };
    TokenStream::from(expanded)
}
