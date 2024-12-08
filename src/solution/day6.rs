use std::collections::HashSet;

use itertools::Itertools;

const INPUT_STRING: &str = include_str!("../../input/day_6");

struct GuardMap {
    starting_location: (usize, usize),
    starting_gaurd_direction: Direction,
    obstac: Vec<(usize, usize)>,
    map: Vec<Vec<((usize, usize), char)>>,
}

impl GuardMap {
    pub fn parse(input: &str) -> GuardMap {
        let mut starting = (0, 0);
        let mut obstac = Vec::new();
        let map = input
            .trim()
            .lines()
            .enumerate()
            .map(|(row_num, l)| {
                l.char_indices()
                    .map(|(col_num, c)| match c {
                        '^' => {
                            starting = (row_num, col_num);
                            ((row_num, col_num), '.')
                        }
                        '#' => {
                            obstac.push((row_num, col_num));
                            ((row_num, col_num), c)
                        }
                        _ => ((row_num, col_num), c),
                    })
                    .collect_vec()
            })
            .collect_vec();
        GuardMap {
            starting_location: starting,
            starting_gaurd_direction: Direction::Up,
            obstac,
            map,
        }
    }

    pub fn generate_events(&mut self) -> u32 {
        // let mut events = Vec::new();
        let mut locs = HashSet::new();
        let mut loc = self.starting_location;
        let mut dir = self.starting_gaurd_direction;
        let row_cnt = self.map.len();
        let col_cnt = self.map[0].len();
        loop {
            // println!("starting loc {:?} dir: {:?}", loc, dir);
            // Check for obstacles
            // add to events and spaces to loc hash

            let mut possible_obs = self
                .obstac
                .iter()
                .filter(|ob| match dir {
                    Direction::Up => ob.1 == loc.1 && ob.0 < loc.0,
                    Direction::Down => ob.1 == loc.1 && ob.0 > loc.0,
                    Direction::Right => ob.0 == loc.0 && ob.1 > loc.1,
                    Direction::Left => ob.0 == loc.0 && ob.1 < loc.1,
                })
                .collect_vec();
            possible_obs.sort_by(|a, b| match dir {
                Direction::Up => b.0.cmp(&a.0),
                Direction::Down => a.0.cmp(&b.0),
                Direction::Right => b.1.cmp(&a.1),
                Direction::Left => a.1.cmp(&b.1),
            });

            // After sorting check if empty.
            // This will indicate that the guard is going to travel off the map
            // Need to calculate distance to end of arr

            if possible_obs.is_empty() {
                let spaces = match dir {
                    Direction::Up => (0..=loc.0)
                        .collect_vec()
                        .iter()
                        .map(|i| (*i, loc.1))
                        .collect_vec(),
                    Direction::Down => (loc.0..row_cnt)
                        .collect_vec()
                        .iter()
                        .map(|i| (*i, loc.1))
                        .collect_vec(),
                    Direction::Right => (loc.1..col_cnt)
                        .collect_vec()
                        .iter()
                        .map(|i| (loc.0, *i))
                        .collect_vec(),
                    Direction::Left => (0..=loc.1)
                        .collect_vec()
                        .iter()
                        .map(|i| (loc.0, *i))
                        .collect_vec(),
                };
                for s in spaces {
                    locs.insert(s);
                }
                // println!("space_set {:?}", locs);

                break;
            }

            // pull first value off of vec (the closest)
            // generate vec of spaces and add

            let closest_ob = possible_obs.first().unwrap();
            // println!("possible {:?}", possible_obs);

            let (spaces, new_loc) = match dir {
                Direction::Up => {
                    dir = Direction::Right;
                    (
                        ((closest_ob.0 + 1)..=loc.0)
                            .collect_vec()
                            .iter()
                            .map(|i| (*i, loc.1))
                            .collect_vec(),
                        (closest_ob.0 + 1, closest_ob.1),
                    )
                }
                Direction::Down => {
                    dir = Direction::Left;
                    (
                        (loc.0..(closest_ob.0))
                            .collect_vec()
                            .iter()
                            .map(|i| (*i, loc.1))
                            .collect_vec(),
                        (closest_ob.0 - 1, closest_ob.1),
                    )
                }
                Direction::Right => {
                    dir = Direction::Down;
                    (
                        (loc.1..(closest_ob.1))
                            .collect_vec()
                            .iter()
                            .map(|i| (loc.0, *i))
                            .collect_vec(),
                        (closest_ob.0, closest_ob.1 - 1),
                    )
                }
                Direction::Left => {
                    dir = Direction::Up;
                    (
                        ((closest_ob.1 + 1)..=loc.1)
                            .collect_vec()
                            .iter()
                            .map(|i| (loc.0, *i))
                            .collect_vec(),
                        (closest_ob.0, closest_ob.1 + 1),
                    )
                }
            };

            loc = new_loc;

            // println!("spaces: {:?} len: {}\n", spaces, spaces.len());
            for s in spaces {
                locs.insert(s);
            }
        }

        locs.len() as u32
    }
}

pub fn part1() -> u32 {
    GuardMap::parse(INPUT_STRING).generate_events()
}

pub fn simple_solution() -> u32 {
    let input = INPUT_STRING;
    let mut map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // A.
    let mut p = (0, 0);
    let mut d = (0, -1);
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    for (y, row) in map.clone().iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '^' {
                p = (x, y);
                map[y][x] = '.';
                break;
            }
        }
    }

    let mut visited = HashSet::<(usize, usize)>::new();
    visited.insert(p);

    loop {
        let (x, y) = p;
        let (dx, dy) = d;
        let next_position = ((x as isize + dx) as usize, (y as isize + dy) as usize);

        if next_position.1 >= map.len() || next_position.0 >= map[0].len() {
            break;
        }

        match map[next_position.1][next_position.0] {
            '#' => {
                d = dirs[(dirs.iter().position(|&_d| _d == d).unwrap() + 1) % dirs.len()];
            }
            '.' => {
                p = next_position;
                visited.insert(p);
            }
            _ => {
                break;
            }
        }
    }

    visited.len() as u32
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Event {
    event_type: bool, //true: collison, false: Exit
    new_direction: (i32, i32),
    location: (usize, usize),
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const TEST: &str = indoc! {"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
        "};

    const TEST_1: &str = indoc! {"
        .....
        ..#..
        ..^.#
        ...#.
        .....
        "};
    const TEST_1_1: &str = indoc! {"
        .....
        ..#..
        ..^.#
        ...#.
        .....
        ..#..
        "};
    const TEST_2: &str = indoc! {"
        ....#
        ...#.
        ..^..
        .....
        ..#..
        "};

    const TEST_3: &str = indoc! {"
        .....
        ..#..
        #.^..
        .....
        .....
        "};
    const TEST_4: &str = indoc! {"
        ......
        ...#..
        ##.^..
        ......
        ......
        "};

    #[test]
    fn events_test_1() {
        let mut g = GuardMap::parse(TEST);
        let res = g.generate_events();

        assert_eq!(res, 41);
    }
    #[test]
    fn events_test_2() {
        let mut g = GuardMap::parse(TEST_1);
        let res = g.generate_events();

        assert_eq!(res, 4);
    }
    #[test]
    fn events_test_between_two_start() {
        let mut g = GuardMap::parse(TEST_1_1);
        let res = g.generate_events();

        assert_eq!(res, 4);
    }
    #[test]
    fn events_test_3() {
        let mut g = GuardMap::parse(TEST_2);
        let res = g.generate_events();

        assert_eq!(res, 3);
    }
    #[test]
    fn events_test_4() {
        let mut g = GuardMap::parse(TEST_3);
        let res = g.generate_events();

        assert_eq!(res, 3);
    }
    #[test]
    fn events_test_5() {
        let mut g = GuardMap::parse(TEST_4);
        let res = g.generate_events();

        assert_eq!(res, 3);
    }
}
