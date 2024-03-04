extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro::TokenTree;
use quote::quote;
use syn::{parse, parse_macro_input, Ident, ItemFn, DeriveInput};

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

#[proc_macro_attribute]
pub fn sh_function_expopt(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut cmd: String = Default::default();
    let mut desc: String = Default::default();
    let input = parse_macro_input!(item as ItemFn);
    let func = &input.sig.ident;
    let mut into_iter = attr.clone().into_iter();
    match attr.clone().into_iter().count() {
        0 =>{
            cmd = func.to_string();
            desc = cmd.clone()+ " cmd";
        },
        1 =>{
            if let Some(TokenTree::Literal(lit)) = into_iter.next() {
                desc = lit.to_string().trim_matches('"').to_string();
            }
            cmd = func.to_string();
        },
        2 =>{
            if let Some(token_tree) = into_iter.next() {
                cmd = token_tree.to_string();
            }
            if let Some(TokenTree::Literal(lit)) = into_iter.next().into_iter().next() {
                desc = lit.to_string().trim_matches('"').to_string();
            }
        },
        3 =>{
            if let Some(token_tree) = into_iter.next() {
                cmd = token_tree.to_string();
            }
            if let Some(TokenTree::Literal(lit)) = into_iter.next().into_iter().next().into_iter().next() {
                desc = lit.to_string().trim_matches('"').to_string();
            }
        },
        _ =>{},
    }
    let static_ident: Ident = syn::parse_str(format!("__fsym_{}", cmd).as_str())
                            .expect("Failed to parse Ident from string");
    let expanded = quote! {
        #[used]
        #[link_section = "FSymTab"]
        static #static_ident: ShSyscall = ShSyscall{
            name:#cmd,
            desc:#desc,
            func:#func,
        };
        #input
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(To)]
pub fn to_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    quote! {
        impl To for #name {
            fn to_const(&self) -> Option<*const()> {
                Some(self as *const #name as *const())
            }
            fn to_mut(&mut self) -> Option<*mut()> {
                Some(self as *mut #name as *mut())
            }
        }
    }.into()
}
