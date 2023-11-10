use aoc_cryptography::Crypto;
use aoc_solution::aoc_solution;

#[aoc_solution(year = 2015, day = 4, part = 1, parser = raw_parser)]
fn part_1(input: &String) -> String {
    let crypto = Crypto::new();
    
    println!("{:?}", crypto.md5("The quick brown fox jumps over the lazy dog".to_string()));

    let needle = [b'0'; 5];
    for i in 100_000.. {
        if crypto.md5(format!("{}{}", input, i)).starts_with(&needle) {
            return format!("{}", i);
        }
    }
    format!("No answer found")
}

#[aoc_solution(year = 2015, day = 4, part = 2, parser = raw_parser)]
fn part_2(input: &String) -> String {
    let crypto = Crypto::new();
    let needle = [b'0'; 6];
    for i in 100_000.. {
        if crypto.md5(format!("{}{}", input, i)).starts_with(&needle) {
            return format!("{}", i);
        }
    }
    format!("No answer found")
}
