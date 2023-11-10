use aoc_acting_closures::ActingClosures;
use aoc_solution::aoc_solution;
use aoc_linear_algebra::vector::Vector;
use aoc_topology::BoundingBox;

#[aoc_solution(year = 2015, day = 2, part = 1, parser = dimensions_parser)]
fn part_1(input: &Vec<Vector<usize, 3>>) -> String {
    format!("{}", input.iter().map(|v| BoundingBox::from(v).apply(|x| x.surfaces().iter().apply(|s| -> usize { s.clone().sum::<usize>() * 2 + s.min().unwrap() })) ).sum::<usize>())
}

#[aoc_solution(year = 2015, day = 2, part = 2, parser = dimensions_parser)]
fn part_2(input: &Vec<Vector<usize, 3>>) -> String {
    format!("{}", input.iter().map(|v| BoundingBox::from(v).apply(|x| { x.volume() + x.min_circumference() }) ).sum::<usize>())
}
