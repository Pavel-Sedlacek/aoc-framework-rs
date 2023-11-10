use proc_macro::{Span, TokenStream};
use quote::{quote};
use syn::{Ident, ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn aoc_solution(attr: TokenStream, item: TokenStream) -> TokenStream {

    let args = parse_macro_input!(attr as AoCMeta);
    let (year, day, part, parser) = (args.year, args.day, args.part, args.parser);
    let name = Ident::new(&format!("year{}_day{}_part{}", year, day, part), Span::call_site().into());
    let func = parse_macro_input!(item as ItemFn);
    let func_name = func.sig.ident.clone();

    let a: TokenStream = quote! {

        pub fn #name() -> String {
            #func

            let raw_input = aoc_input_loader::aoc_input_loader(#year, #day).load(false);
            let input = aoc_parsers::#parser::#parser(&raw_input);

            #func_name(&input)
        }
    }.into();

    a.into()
}

use attribute_derive::Attribute;

#[derive(Attribute)]
#[attribute(ident = aoc_solution)]
struct AoCMeta {
    part: isize,
    day: isize,
    year: isize,
    parser: Ident
}

