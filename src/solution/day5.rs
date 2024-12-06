use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT_STRING: &str = include_str!("../../input/day_5");
fn parse(input: &str) -> (&str, &str) {
    input.trim().split_once("\n\n").unwrap()
}

fn do_part1(input: &str) -> u32 {
    let (rules_block, printing_order) = parse(input);

    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut before_rules: HashMap<&str, Vec<&str>> = HashMap::new();

    for rule in rules_block.trim().lines() {
        let (key, value) = rule.trim().split_once("|").unwrap();
        rules
            .entry(key)
            .and_modify(|v| v.push(value))
            .or_insert(vec![value]);
        before_rules
            .entry(value)
            .and_modify(|v| v.push(key))
            .or_insert(vec![key]);
    }

    let valid = printing_order
        .trim()
        .lines()
        .filter_map(|order_line| {
            let order_vec = order_line.split(",").collect_vec();
            for (i, p_order) in order_vec.iter().enumerate() {
                //first check every order after you (easiest)
                // requires getting rule and checking all vec after

                let valid_flag = rules.get(p_order).is_none_or(|val_vec| {
                    val_vec
                        .iter()
                        .filter(|x| order_vec[..i].contains(x))
                        .collect_vec()
                        .is_empty()
                });
                if !valid_flag {
                    return None;
                }
            }
            Some(order_vec)
        })
        .collect_vec();
    valid
        .iter()
        .map(|v| v.get(v.len() / 2).unwrap().parse::<u32>().unwrap())
        .sum()
}

fn do_part2(input: &str) -> u32 {
    let (rules_block, printing_order) = parse(input);

    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    for rule in rules_block.trim().lines() {
        let (key, value) = rule.trim().split_once("|").unwrap();
        rules
            .entry(key.parse().unwrap())
            .or_default()
            .insert(value.parse().unwrap());
    }

    let mut invalid = printing_order
        .trim()
        .lines()
        .filter_map(|order_line| {
            let order_vec = order_line
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec();
            for (i, p_order) in order_vec.iter().enumerate() {
                //first check every order after you (easiest)
                // requires getting rule and checking all vec after

                let valid_flag = rules.get(p_order).is_none_or(|val_vec| {
                    val_vec
                        .iter()
                        .filter(|x| order_vec[..i].contains(x))
                        .collect_vec()
                        .is_empty()
                });
                if !valid_flag {
                    return Some(order_vec);
                }
            }
            None
        })
        .collect_vec();
    let ivec = invalid
        .iter_mut()
        .map(|i| {
            i.sort_unstable_by(|a, b| (rules.contains_key(b) && rules[b].contains(a)).cmp(&true));
            i
        })
        .collect_vec();

    ivec.iter().map(|v| v.get(v.len() / 2).unwrap()).sum()
}

pub fn part1() -> u32 {
    do_part1(INPUT_STRING)
}

pub fn part2() -> u32 {
    do_part2(INPUT_STRING)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const TEST: &str = indoc! {"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13
            
            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47 
        "};

    #[test]
    fn part1() {
        let temp = do_part1(TEST);

        assert_eq!(143, temp);
    }

    #[test]
    fn part2() {
        let temp = do_part2(TEST);

        assert_eq!(123, temp);
    }
}
