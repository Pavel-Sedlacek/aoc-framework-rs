use aoc_solution::aoc_solution;
use aoc_iter_tools::{CharsToString, GrowingWindow, Predicates};

#[aoc_solution(year = 2015, day = 1, part = 1, parser = raw_parser)]
fn part_1(input: &String) -> String {
    format!("{}", input.matches("(").count() as isize - input.matches(")").count() as isize)
}

#[aoc_solution(year = 2015, day = 1, part = 2, parser = raw_parser)]
fn part_2(input: &String) -> String {
    let mut counters = (0, 0);
    for i in input.chars() {
        counters.1 += 1;
        match i {
            '(' => { counters.0 += 1},
            _ => { counters.0 -= 1}
        }
        if counters.0 == -1 { return format!("{}", counters.1) }
    }
    format!("No solution found")
}

