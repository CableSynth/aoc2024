use itertools::Itertools;
use regex::Regex;

const INPUT_STRING: &str = include_str!("../../input/day_3");

fn parse_1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();
    let temp = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [c]) = caps.extract();
            c
        })
        .collect_vec();
    temp.iter()
        .map(|x| {
            x.split(",")
                .map(|i| i.parse::<u32>().unwrap())
                .collect_vec()
                .iter()
                .product::<u32>()
        })
        .sum()
}

pub fn parse_2(input: &str) -> u32 {
    let re = Regex::new(r"(don't\(\))|(do\(\))|mul\((\d{1,3},\d{1,3})\)").unwrap();
    let temp = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [c]) = caps.extract();
            c
        })
        .collect_vec();
    let mut should_mul = true;
    let mut c = Vec::new();
    for s in temp {
        if s == "do()" {
            should_mul = true;
        } else if s == "don't()" {
            should_mul = false
        } else if should_mul {
            c.push(s);
        }
    }
    c.iter()
        .map(|x| {
            x.split(",")
                .map(|i| i.parse::<u32>().unwrap())
                .collect_vec()
                .iter()
                .product::<u32>()
        })
        .sum()
}

pub fn part1() -> u32 {
    parse_1(INPUT_STRING)
}

pub fn part2() -> u32 {
    parse_2(INPUT_STRING)
}

mod test {

    use super::{parse_1, parse_2};

    const TEST_CASE: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_CASE_bad_long_number: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(112,8)mul(8,5))";
    const TEST_CASE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn parse_test() {
        let vec_out = parse_1(TEST_CASE);
        println!("{:?}", vec_out);
    }
    #[test]
    fn parse_test_bad_number_too_long() {
        let vec_out = parse_1(TEST_CASE_bad_long_number);
        println!("{:?}", vec_out);
    }
    #[test]
    fn parse2_test() {
        let vec_out = parse_2(TEST_CASE_2);
        println!("{:?}", vec_out);
    }
}
