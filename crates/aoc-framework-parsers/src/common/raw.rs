use aoc_framework_macros::aoc_parser;

#[aoc_parser(name = "raw_parser")]
fn raw(a: &String) -> String {
  String::from(a)
}