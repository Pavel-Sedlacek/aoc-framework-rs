use proc_macro::{Span, TokenStream};
use quote::{quote};
use syn::{Ident, ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn aoc_parser(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AoCParserMeta);
    let name = Ident::new(&args.name, Span::call_site().into());
    let func = parse_macro_input!(input as ItemFn);
    let func_name = func.sig.ident.clone();
    let func_return = func.sig.output.clone();

    let a: TokenStream = quote! {

        pub fn #name(raw_input: &String) #func_return {
            #func

            #func_name(raw_input)
        }
    }.into();

    a.into()
}

use attribute_derive::{Attribute};

#[derive(Attribute)]
#[attribute(ident = aoc_parser)]
struct AoCParserMeta {
    name: String
}
