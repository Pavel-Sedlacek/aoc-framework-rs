use std::collections::HashSet;
use aoc_solution::aoc_solution;

#[aoc_solution(year = 2015, day = 3, part = 1, parser = raw_parser)]
fn part_1(input: &String) -> String {
    let mut santa = (HashSet::new(), (0, 0));
    for c in input.chars() {
        match c {
            'v' => { santa.1.1 += 1; santa.0.insert(santa.1.clone()); }
            '^' => { santa.1.1 -= 1; santa.0.insert(santa.1.clone()); }
            '>' => { santa.1.0 += 1; santa.0.insert(santa.1.clone()); }
            _   => { santa.1.0 -= 1; santa.0.insert(santa.1.clone()); }
        }
    }
    format!("{}", santa.0.len())
}

#[aoc_solution(year = 2015, day = 3, part = 2, parser = raw_parser)]
fn part_2(input: &String) -> String {
    let mut santas = (HashSet::new(), [(0, 0), (0, 0)]);
    for i in 0..=1 {
        for c in input.chars().skip(i).step_by(2) {
            match c {
                'v' => { santas.1[i].1 += 1; santas.0.insert(santas.1[i].clone()); }
                '^' => { santas.1[i].1 -= 1; santas.0.insert(santas.1[i].clone()); }
                '>' => { santas.1[i].0 += 1; santas.0.insert(santas.1[i].clone()); }
                _   => { santas.1[i].0 -= 1; santas.0.insert(santas.1[i].clone()); }
            }
        }
    }
    format!("{}", santas.0.len())
}