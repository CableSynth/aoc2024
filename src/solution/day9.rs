use std::collections::VecDeque;

use itertools::Itertools;

const INPUT_STRING: &str = include_str!("../../input/day_9");
struct DiskMap {
    disk_layout: VecDeque<File>,
    spaces: Vec<u64>,
}

#[derive(Clone, Copy, Debug)]
struct File {
    blocks: u64,
    id: u64,
    is_space: bool,
}

pub fn part1() -> u64 {
    solve_1(INPUT_STRING)
}

pub fn part2() -> u64 {
    solve_2(INPUT_STRING)
}

fn solve_1(input: &str) -> u64 {
    let mut i = 0;
    DiskMap::parse(input, false)
        .defrag()
        .iter()
        .map(|f| {
            let r = (0..f.blocks).into_iter().fold(0, |acc, x| {
                let pos = i + x;
                acc + (f.id * pos)
            });
            i += f.blocks;
            r
        })
        .sum()
}

fn solve_2(input: &str) -> u64 {
    let mut f = Files::parse(input);
    f.file_reorder();
    f.check_sum()
}

impl DiskMap {
    fn parse(input: &str, part2: bool) -> Self {
        let mut spaces = Vec::new();
        let mut reading_file = true;
        let mut current_id = 0;

        let disk_layout = input
            .trim()
            .chars()
            .filter_map(|c| {
                let blocks = c.to_digit(10).unwrap() as u64;
                if reading_file {
                    reading_file = false;
                    let file = Some(File {
                        blocks,
                        id: current_id,
                        is_space: false,
                    });
                    current_id += 1;
                    return file;
                } else {
                    reading_file = true;
                    if part2 {
                        return Some(File {
                            blocks,
                            id: 0,
                            is_space: true,
                        });
                    }
                    spaces.push(blocks);
                    None
                }
            })
            .collect::<VecDeque<File>>();
        spaces.reverse();

        Self {
            disk_layout,
            spaces,
        }
    }

    fn defrag(&mut self) -> Vec<File> {
        let mut result: Vec<File> = Vec::new();
        let mut front_back = true;
        let mut back = self.disk_layout.pop_back();

        loop {
            if front_back {
                front_back = false;
                if let Some(f) = self.disk_layout.pop_front() {
                    result.push(f);
                } else {
                    if let Some(f) = back {
                        result.push(f);
                    }
                    break;
                }
            } else {
                if let Some(f) = back {
                    // println!("back: {:?}", f);
                    let s = self.spaces.pop();
                    if let Some(mut space) = s {
                        let new_file_part = if space > f.blocks {
                            // println!("space greater {:?} f: {:?}", space, f);
                            space = space - f.blocks;
                            self.spaces.push(space);
                            back = self.disk_layout.pop_back();
                            File {
                                blocks: f.blocks,
                                id: f.id,
                                is_space: false,
                            }
                        } else {
                            let new_block_sz = f.blocks - space;
                            front_back = true;
                            if new_block_sz == 0 {
                                back = self.disk_layout.pop_back();
                                File {
                                    blocks: f.blocks,
                                    id: f.id,
                                    is_space: false,
                                }
                            } else {
                                back = Some(File {
                                    blocks: new_block_sz,
                                    id: f.id,
                                    is_space: false,
                                });
                                File {
                                    blocks: space,
                                    id: f.id,
                                    is_space: false,
                                }
                            }
                        };
                        result.push(new_file_part);
                    } else {
                        // println!(
                        //     "no spaces left! back {:?}, f: {:?}, self.disk {:?}",
                        //     back, f, self.disk_layout
                        // );
                        result.push(f);
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        result
    }
}

struct Files {
    files: Vec<File_2>,
}

#[derive(Clone, Copy, Debug)]
struct File_2 {
    position: u32,
    size: u8,
    id: u32,
}

impl Files {
    fn parse(input: &str) -> Self {
        let mut id = 0;
        let mut position = 0;
        let files = input
            .trim()
            .char_indices()
            .filter_map(|(i, c)| {
                let mut file_r = None;
                let size = c.to_digit(10).unwrap() as u8;

                if i % 2 == 0 {
                    file_r = Some(File_2 { position, size, id });
                    id += 1;
                }
                position += size as u32;

                return file_r;
            })
            .collect_vec();

        Self { files }
    }
    // fn parse(input: &str) -> Self {
    //     let mut files = Vec::new();
    //     let (mut id, mut position) = (0, 0);

    //     for (idx, chr) in input.trim().char_indices() {
    //         let size = chr.to_digit(10).unwrap() as u8;

    //         if idx % 2 == 0 {
    //             files.push(File_2 { position, size, id });
    //             id += 1;
    //         }

    //         position += size as u32;
    //     }

    //     Self { files }
    // }

    fn file_reorder(&mut self) {
        let max_id = self.files.last().unwrap().id;

        for id in (0..=max_id).rev() {
            let file_idx = self.files.iter().position(|f| f.id == id).unwrap();
            let file = &self.files[file_idx];

            let mut new_pos = None;
            for (start_file, end_file) in self.files.iter().tuple_windows() {
                let free_space =
                    (end_file.position) - (start_file.position + start_file.size as u32);
                let pos = start_file.position + start_file.size as u32;
                if pos > file.position {
                    break;
                }

                if free_space >= file.size as u32 {
                    new_pos = Some(pos);
                    break;
                }
            }

            if let Some(new_pos) = new_pos {
                self.files[file_idx].position = new_pos;
            }
            self.files.sort_by_key(|f| f.position);
        }
    }

    pub fn check_sum(&self) -> u64 {
        self.files
            .iter()
            .map(|f| {
                let s = (f.position..(f.size as u32 + f.position as u32))
                    .into_iter()
                    .fold(0, |acc, size| acc + size as u64 * f.id as u64);
                s
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../../test_input/day_9_1");

    #[test]
    fn test_defrag() {
        let out = DiskMap::parse(TEST, false).defrag();
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(1928, solve_1(TEST));
    }
    #[test]
    fn test_solve_2() {
        assert_eq!(2858, solve_2(TEST));
    }
}
