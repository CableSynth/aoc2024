use itertools::Itertools;

const INPUT_STRING: &str = include_str!("../../input/day_7");

pub fn do_part1(input: &str) -> u64 {
    BridgeCalc::parse(input)
        .problems
        .iter()
        .filter_map(|p| if p.evaluate(false) { Some(p.res) } else { None })
        .sum()
}
pub fn do_part2(input: &str) -> u64 {
    BridgeCalc::parse(input)
        .problems
        .iter()
        .filter_map(|p| if p.evaluate(true) { Some(p.res) } else { None })
        .sum()
}

pub fn part1() -> u64 {
    do_part1(INPUT_STRING)
}

pub fn part2() -> u64 {
    do_part2(INPUT_STRING)
}

struct BridgeCalc {
    problems: Vec<Problem>,
}

struct Problem {
    res: u64,
    input: Vec<u64>,
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Mult,
    Concat,
}

impl BridgeCalc {
    fn parse(input: &str) -> Self {
        let problems = input
            .trim()
            .lines()
            .map(|line| {
                let (res, inputs) = line.split_once(": ").unwrap();
                let res = res.parse::<u64>().unwrap();

                let inputs = inputs
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect_vec();
                Problem { res, input: inputs }
            })
            .collect_vec();

        BridgeCalc { problems }
    }
}

impl Problem {
    fn evaluate(&self, do_p2: bool) -> bool {
        let num_ops = self.input.len() - 1;
        let mut ops = vec![0; num_ops];
        'outer: loop {
            let mut result = self.input[0];
            for (&op, &num) in ops.iter().zip(self.input.iter().skip(1)) {
                let preform = [Operation::Add, Operation::Mult, Operation::Concat][op];
                result = preform.compute(result, num);
                // dbg!(result);
            }

            if result == self.res {
                return true;
            }

            for i in 0..ops.len() {
                ops[i] += 1;
                if ops[i] <= (1 + do_p2 as usize) {
                    // go to next loop for each number
                    continue 'outer;
                }

                // reset for next loop
                ops[i] = 0;
            }

            return false;
        }
    }
}

impl Operation {
    fn compute(&self, result: u64, next_num: u64) -> u64 {
        match self {
            Self::Add => result + next_num,
            Self::Mult => result * next_num,
            Self::Concat => {
                let mut temp = next_num;
                let mut output = result;
                while temp > 0 {
                    temp /= 10;
                    output *= 10;
                }
                output + next_num
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;
    const TEST: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
        "};

    #[test]
    fn test_evaluate() {
        assert_eq!(do_part1(TEST), 3749);
    }
    #[test]
    fn test_evaluate2() {
        assert_eq!(do_part2(TEST), 11387);
    }
}
