use itertools::Itertools;
use nd_vec::{vector, Vec2};
const INPUT_STRING: &str = include_str!("../../input/day_13");

fn do_part1(input: &str) -> usize {
    ClawMachine::parse(input).solve()
}

fn do_part2(input: &str) -> usize {
    ClawMachine::parse(input).part2()
}

pub fn part1() -> usize {
    do_part1(INPUT_STRING)
}

pub fn part2() -> usize {
    do_part2(INPUT_STRING)
}

struct ClawMachine {
    cases: Vec<Case>,
}

struct Case {
    button_a: Vec2<usize>,
    button_b: Vec2<usize>,

    goal: Vec2<usize>,
}

impl ClawMachine {
    fn parse(input: &str) -> Self {
        let cases = input
            .split("\n\n")
            .map(|case_chunk| {
                let mut lines = case_chunk.lines();
                let button_a = parse_moves(lines.next().unwrap());
                let button_b = parse_moves(lines.next().unwrap());
                let (_, goal) = lines.next().unwrap().split_once(": ").unwrap();
                let (goalx, goaly) = goal.split_once(", ").unwrap();
                let goal = vector!(goalx[2..].parse().unwrap(), goaly[2..].parse().unwrap());
                Case {
                    button_a,
                    button_b,
                    goal,
                }
            })
            .collect_vec();
        Self { cases }
    }

    fn solve(&self) -> usize {
        self.cases.iter().map(|c| c.best_moves()).sum()
    }

    fn part2(&mut self) -> usize {
        self.cases
            .iter_mut()
            .map(|c| {
                c.goal += vector!(10000000000000, 10000000000000);
                c.best_moves()
            })
            .sum()
    }
}

impl Case {
    fn best_moves(&self) -> usize {
        let cast_fn = |c: Vec2<usize>| c.try_cast::<i64>().unwrap();
        let (g, a, b) = (
            cast_fn(self.goal),
            cast_fn(self.button_a),
            cast_fn(self.button_b),
        );

        let (gx, gy) = (g.x(), g.y());
        let (ax, ay) = (a.x(), a.y());
        let (bx, by) = (b.x(), b.y());

        // wolfram alpha
        // solve g = z a + y b
        // w = t a + c b for a

        let a = (by * gx - bx * gy) / (ax * by - ay * bx);
        let b = (gx - ax * a) / bx;

        if a <= 0 || b <= 0 || self.goal != self.button_a * a as usize + self.button_b * b as usize
        {
            return 0;
        }
        // a is three times the cost of b
        a as usize * 3 + b as usize
    }
}

fn parse_moves(input: &str) -> Vec2<usize> {
    let (_, locs) = input.split_once(": ").unwrap();
    let (x, y) = locs.split_once(", ").unwrap();
    vector!(x[1..].parse().unwrap(), y[1..].parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../../test_input/day_13_1");
    #[test]
    fn test_p1() {
        assert_eq!(480, do_part1(TEST));
    }
    #[test]
    fn test_p2() {
        assert_eq!(480, do_part2(TEST));
    }
}
