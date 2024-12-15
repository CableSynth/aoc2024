use std::collections::HashSet;

use itertools::Itertools;
use nd_vec::{vector, Vec2};

const INPUT_STRING: &str = include_str!("../../input/day_12");

struct Garden {
    // TODO: Make something like a matrix
    grid: Vec<Vec<char>>,
    seen: HashSet<Vec2<usize>>,
}

fn solve_1(input: &str) -> usize {
    let mut g = Garden::parse(input);
    g.points()
        .filter_map(|loc| g.search(loc))
        .map(|(area, p)| area.len() * p)
        .sum()
}

// TODO: Solve this
fn solve_2(input: &str) -> usize {
    let mut g = Garden::parse(input);
    g.points()
        .filter_map(|loc| g.search(loc))
        .map(|(area, _)| {
            println!("area: {:?}", area);
            0
        })
        .sum()
}

pub fn part1() -> usize {
    solve_1(INPUT_STRING)
}

impl Garden {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        Self {
            grid,
            seen: HashSet::new(),
        }
    }
    fn points(&self) -> impl Iterator<Item = Vec2<usize>> {
        let row_size = self.grid.len();
        (0..row_size)
            .cartesian_product(0..self.grid[0].len())
            .map(|(x, y)| vector!(x, y))
    }

    fn search(&mut self, start: Vec2<usize>) -> Option<(HashSet<Vec2<usize>>, usize)> {
        // let plant_under_test = self.grid[start.x()][start.y()];
        // println!("under_test: {}", plant_under_test);
        if !self.seen.insert(start) {
            // println!("seen location already: {:?}", start);
            return None;
        }

        // Need to start search
        let mut search_q = Vec::new();
        let mut perimiter = 0;

        let plant_under_test = self.grid[start.x()][start.y()];
        let mut area = HashSet::new();
        area.insert(start);
        search_q.push(start);
        let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

        while let Some(next) = search_q.pop() {
            let valid = directions
                .iter()
                .map(|(row_dir, col_dir)| (row_dir + next.x() as i32, col_dir + next.y() as i32))
                .filter(|(r, c)| {
                    if !r.is_negative() && !c.is_negative() {
                        true
                    } else {
                        perimiter += 1;
                        false
                    }
                })
                .map(|(r, c)| (r as usize, c as usize))
                .collect_vec();

            for (n_row, n_col) in valid {
                if let Some(val) = self.grid.get(n_row as usize) {
                    if let Some(plant) = val.get(n_col as usize) {
                        let v = vector!(n_row, n_col);
                        if *plant == plant_under_test {
                            if self.seen.insert(v) {
                                area.insert(v);
                                search_q.push(v);
                            }
                        } else {
                            perimiter += 1;
                        }
                    } else {
                        perimiter += 1;
                    }
                } else {
                    perimiter += 1;
                }
            }
        }
        // println!(
        //     "area found: {:?} len: {}, per: {}",
        //     area,
        //     area.len(),
        //     perimiter
        // );

        Some((area, perimiter))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../../test_input/day_12_1");
    #[test]
    fn test_p1() {
        assert_eq!(1930, solve_1(TEST))
    }
    #[test]
    fn test_p2() {
        assert_eq!(1930, solve_2(TEST))
    }
}
