use aoc_linear_algebra::vector::Vector;
use aoc_parser::aoc_parser;

#[aoc_parser(name = "dimensions_parser")]
fn dimensions(a: &String) -> Vec<Vector<usize, 3>> {
    a.lines().map(|line| {
        let components = line.split("x").map(|val| val.parse::<usize>().unwrap()).collect::<Vec<_>>();
        Vector::new([components[0], components[1], components[2]])
    }).collect()
}