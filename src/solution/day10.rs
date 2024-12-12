use std::collections::{BinaryHeap, VecDeque};

use itertools::Itertools;

const INPUT_STRING: &str = include_str!("../../input/day_10");

pub fn part1() -> u64 {
    HikingMap::parse(INPUT_STRING).hiking_path()
}
pub fn part2() -> u64 {
    HikingMap::parse(INPUT_STRING).hiking_path_2()
}

struct HikingMap {
    starts: Vec<(u32, u32)>,
    map: Vec<Vec<u8>>,
}

impl HikingMap {
    fn parse(input: &str) -> Self {
        let mut starts = Vec::new();
        let map = input
            .trim()
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.char_indices()
                    .map(|(col, c)| {
                        let num = c.to_digit(10).unwrap() as u8;
                        if num == 0 {
                            starts.push((row as u32, col as u32));
                        }
                        num
                    })
                    .collect_vec()
            })
            .collect_vec();
        Self { starts, map }
    }

    fn hiking_path(&self) -> u64 {
        let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        self.starts
            .iter()
            .map(|s| {
                let mut count = 0;
                let mut path_q = VecDeque::new();
                // Remember to use cmp reverse

                path_q.push_back(*s);

                while !path_q.is_empty() {
                    let (row, col) = path_q.pop_front().unwrap();
                    let under_test = self.map[row as usize][col as usize];
                    if self.map[row as usize][col as usize] == 9 {
                        count += 1;
                        continue;
                    }
                    let valid = directions
                        .iter()
                        .map(|(row_dir, col_dir)| (row_dir + row as i32, col_dir + col as i32))
                        .filter(|(r, c)| !r.is_negative() && !c.is_negative())
                        .map(|(r, c)| (r as u32, c as u32))
                        .collect_vec();

                    for (n_row, n_col) in valid {
                        if let Some(val) = self.map.get(n_row as usize) {
                            if let Some(num) = val.get(n_col as usize) {
                                if *num == under_test + 1 && !path_q.contains(&(n_row, n_col)) {
                                    path_q.push_back((n_row, n_col));
                                }
                            }
                        }
                    }
                }
                count
            })
            .sum()
    }

    fn hiking_path_2(&self) -> u64 {
        let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        self.starts
            .iter()
            .map(|s| {
                let mut count = 0;
                let mut path_q = VecDeque::new();
                // Remember to use cmp reverse

                path_q.push_back(*s);

                while !path_q.is_empty() {
                    let (row, col) = path_q.pop_front().unwrap();
                    let under_test = self.map[row as usize][col as usize];
                    if self.map[row as usize][col as usize] == 9 {
                        count += 1;
                        continue;
                    }
                    let valid = directions
                        .iter()
                        .map(|(row_dir, col_dir)| (row_dir + row as i32, col_dir + col as i32))
                        .filter(|(r, c)| !r.is_negative() && !c.is_negative())
                        .map(|(r, c)| (r as u32, c as u32))
                        .collect_vec();

                    for (n_row, n_col) in valid {
                        if let Some(val) = self.map.get(n_row as usize) {
                            if let Some(num) = val.get(n_col as usize) {
                                if *num == under_test + 1 {
                                    path_q.push_back((n_row, n_col));
                                }
                            }
                        }
                    }
                }
                count
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../../test_input/day_10_1");
    #[test]
    fn test_hike() {
        assert_eq!(36, HikingMap::parse(TEST).hiking_path());
    }
}
