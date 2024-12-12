use std::collections::HashMap;

const INPUT_STRING: &str = include_str!("../../input/day_11");
struct Stone {
    stones: Vec<u64>,
}

pub fn part1() -> u64 {
    Stone::parse(INPUT_STRING).solve(25)
}
pub fn part2() -> u64 {
    Stone::parse(INPUT_STRING).solve(75)
}

impl Stone {
    fn parse(input: &str) -> Self {
        let stones = input
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        Self { stones }
    }

    fn solve(self, num_blinks: u32) -> u64 {
        let mut count: HashMap<u64, u64> = HashMap::new();
        self.stones
            .into_iter()
            .for_each(|v| *count.entry(v).or_default() += 1);

        for _ in 0..num_blinks {
            let mut new_map = HashMap::new();
            for (stone, num) in count {
                if stone == 0 {
                    *new_map.entry(1).or_default() += num;
                } else if let Some((a, b)) = split_digits(stone) {
                    *new_map.entry(a).or_default() += num;
                    *new_map.entry(b).or_default() += num;
                } else {
                    *new_map.entry(stone * 2024).or_default() += num;
                }
            }
            count = new_map;
        }
        count.values().sum()
    }
}

fn split_digits(num: u64) -> Option<(u64, u64)> {
    let digits = num.ilog10() + 1;
    let pow = 10_u64.pow(digits / 2);
    (digits & 1 == 0).then(|| (num / pow, num % pow))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../../test_input/day_11_1");

    #[test]
    fn test_1() {
        assert_eq!(55312, Stone::parse(TEST).solve(25))
    }
}
