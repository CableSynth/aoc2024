use std::collections::BinaryHeap;

use itertools::Itertools;

const INPUT_STRING: &str = include_str!("../../input/day_1");

fn read_input(input: &str) -> (BinaryHeap<u32>, BinaryHeap<u32>) {
    let mut right_heap = BinaryHeap::new();
    let mut left_heap = BinaryHeap::new();
    for l in input.lines() {
        let split = l.trim().split_whitespace().collect_vec();
        right_heap.push(split[1].parse::<u32>().unwrap());
        left_heap.push(split[0].parse::<u32>().unwrap());
    }
    (left_heap, right_heap)
}

pub fn part1() -> u32 {
    let (left_heap, right_heap) = read_input(INPUT_STRING);
    let left_list = left_heap.into_sorted_vec();
    let right_list = right_heap.into_sorted_vec();
    let s = left_list
        .iter()
        .rev()
        .zip(right_list.iter().rev())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .collect::<Vec<u32>>();
    s.iter().sum()
}

pub fn part2() -> u32 {
    let (left_heap, right_heap) = read_input(INPUT_STRING);
    let left_list = left_heap.into_sorted_vec();
    let right_list = right_heap.into_sorted_vec();

    left_list
        .iter()
        .map(|lv| lv * (right_list.iter().filter(|r| *r == lv).count()) as u32)
        .collect_vec()
        .iter()
        .sum()
}
