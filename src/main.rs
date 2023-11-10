use aoc_finalize::aoc_finalize;
use aoc_initialize::aoc_initialize;
use aoc_year::aoc_year;

fn main() {
    aoc_initialize!();

    println!("{}", aoc_2015::d1::year2015_day1_part1());
    println!("{}", aoc_2015::d1::year2015_day1_part2());
    println!("{}", aoc_2015::d2::year2015_day2_part1());
    println!("{}", aoc_2015::d2::year2015_day2_part2());
    println!("{}", aoc_2015::d3::year2015_day3_part1());
    println!("{}", aoc_2015::d3::year2015_day3_part2());
    println!("{}", aoc_2015::d4::year2015_day4_part1());
    println!("{}", aoc_2015::d4::year2015_day4_part2());

    aoc_finalize!();
}

