use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT_STRING: &str = include_str!("../../input/day_8");

pub fn part1() -> u32 {
    Map::parse(INPUT_STRING, false).solve(false)
}
pub fn part2() -> u32 {
    Map::parse(INPUT_STRING, true).solve(true)
}

struct Map {
    antenna: HashMap<char, HashSet<(u32, u32)>>,
    antinodes: HashSet<(u32, u32)>,
    dimensions: (u32, u32),
}

impl Map {
    fn parse(input: &str, partb: bool) -> Self {
        let mut antenna: HashMap<char, HashSet<(u32, u32)>> = HashMap::new();
        let mut antinodes: HashSet<_> = HashSet::new();
        let l = input.lines().collect_vec();
        let row_cnt = l.len();
        let col_cnt = l[0].chars().count();

        for (row, lin) in l.iter().enumerate() {
            for (col, c) in lin.char_indices() {
                if c != '.' {
                    antenna
                        .entry(c)
                        .or_default()
                        .insert((row as u32, col as u32));
                    if partb {
                        antinodes.insert((row as u32, col as u32));
                    }
                }
            }
        }

        Self {
            antenna,
            antinodes,
            dimensions: (row_cnt as u32, col_cnt as u32),
        }
    }

    fn solve(&mut self, partb: bool) -> u32 {
        for k in self.antenna.keys() {
            for (a, b) in self.antenna.get(k).unwrap().iter().tuple_combinations() {
                let ia = (a.0 as i32, a.1 as i32);
                let ib = (b.0 as i32, b.1 as i32);
                // slope for closet to a
                let slope1 = (ia.0 - ib.0, ia.1 - ib.1);
                let mut anit_1 = (2 * ia.0 - ib.0, 2 * ia.1 - ib.1);
                //slope for closest to b
                let slope2 = (ib.0 - ia.0, ib.1 - ia.1);
                let mut anit_2 = (2 * ib.0 - ia.0, 2 * ib.1 - ia.1);

                if anit_1.0 < self.dimensions.0 as i32
                    && anit_1.1 < self.dimensions.1 as i32
                    && anit_1.1 > -1
                    && anit_1.0 > -1
                {
                    self.antinodes.insert((anit_1.0 as u32, anit_1.1 as u32));
                }
                if anit_2.0 < self.dimensions.0 as i32
                    && anit_2.1 < self.dimensions.1 as i32
                    && anit_2.1 > -1
                    && anit_2.0 > -1
                {
                    self.antinodes.insert((anit_2.0 as u32, anit_2.1 as u32));
                }
                if partb {
                    loop {
                        anit_1 = (anit_1.0 + slope1.0, anit_1.1 + slope1.1);
                        if anit_1.0 < self.dimensions.0 as i32
                            && anit_1.1 < self.dimensions.1 as i32
                            && anit_1.1 > -1
                            && anit_1.0 > -1
                        {
                            self.antinodes.insert((anit_1.0 as u32, anit_1.1 as u32));
                        } else {
                            break;
                        }
                    }
                    loop {
                        anit_2 = (anit_2.0 + slope2.0, anit_2.1 + slope2.1);
                        if anit_2.0 < self.dimensions.0 as i32
                            && anit_2.1 < self.dimensions.1 as i32
                            && anit_2.1 > -1
                            && anit_2.0 > -1
                        {
                            self.antinodes.insert((anit_2.0 as u32, anit_2.1 as u32));
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        // println!("antinodes: {:#?}", self.antinodes);
        self.antinodes.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../../test_input/day_8");

    #[test]
    fn test_p1_solve() {
        assert_eq!(14, Map::parse(TEST, false).solve(false))
    }
    #[test]
    fn test_p2_solve() {
        assert_eq!(34, Map::parse(TEST, true).solve(true))
    }
}
