use itertools::Itertools;

const INPUT_STRING: &str = include_str!("../../input/day_2");
const TEST: &str = include_str!("../../test_input/day_2_1");
const TEST_EDGE: &str = include_str!("../../test_input/day_2_2");

fn read_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect_vec()
}

// Need to rewrite with tuble windows
pub fn part1() -> usize {
    let input = read_input(INPUT_STRING);
    input
        .iter()
        .filter(|x| {
            let sign = (x[0] - x[1]).signum();
            x.iter()
                .tuple_windows()
                .map(|(a, b)| a - b)
                .all(|i| (1..=3).contains(&i.abs()) && i.signum() == sign)
        })
        .count()
}

pub fn part2() -> usize {
    let input = read_input(INPUT_STRING);
    input.iter().filter(|report| safe(report, None)).count()
}

fn safe(input: &[i32], skip: Option<usize>) -> bool {
    let vals = input
        .iter()
        .enumerate()
        .filter(|(idx, _)| skip.is_none() || Some(*idx) != skip)
        .map(|(_, &x)| x);
    let mut difference = vals.tuple_windows().map(|(a, b)| a - b).peekable();
    let sign = difference.peek().unwrap().signum();
    let first_bad = difference.position(|x| !(1..=3).contains(&x.abs()) || x.signum() != sign);
    match first_bad {
        Some(x) if skip.is_none() => {
            safe(input, Some(x + 1))
                || safe(input, Some(x.saturating_sub(1)))
                || safe(input, Some(x))
        }
        None => true,
        _ => false,
    }
}
