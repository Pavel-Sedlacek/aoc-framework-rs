use attribute_derive::Attribute;
use proc_macro::{TokenStream, Span};
use syn::{parse_macro_input, ItemFn, Ident};
use quote::quote;

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

#[derive(Attribute)]
#[attribute(ident = aoc_solution)]
struct AoCMeta {
    part: isize,
    day: isize,
    year: isize,
    parser: Ident
}

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


#[derive(Attribute)]
#[attribute(ident = aoc_parser)]
struct AoCParserMeta {
    name: String
}