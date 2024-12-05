use itertools::Itertools;

const INPUT_STRING: &str = include_str!("../../input/day_4");

fn parse(input: &str) -> Vec<Vec<(usize, char)>> {
    input
        .lines()
        .map(|l| l.trim().char_indices().collect_vec())
        .collect_vec()
}

fn do_part1(input: &str) -> u32 {
    let input_array = parse(input);
    let check_array = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let x_indicies = input_array
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            c.iter()
                .filter_map(|(i, ch)| if *ch == 'X' { Some((idx, *i)) } else { None })
                .collect_vec()
        })
        .flatten()
        .collect_vec();
    // println!("{:?}", x_indicies);
    x_indicies.iter().fold(0, |acc, x| {
        acc + check_neighbors(&input_array, *x, check_array.to_vec(), 'M')
    })
}

fn do_part2(input: &str) -> u32 {
    let input_array = parse(input);
    let check_array = [(-1, -1), (1, 1), (1, -1), (-1, 1)];
    let x_indicies = input_array
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            c.iter()
                .filter_map(|(i, ch)| if *ch == 'A' { Some((idx, *i)) } else { None })
                .collect_vec()
        })
        .flatten()
        .collect_vec();
    x_indicies.iter().fold(0, |acc, x| {
        let mut temp_vec = Vec::new();

        for (dline, dchar) in check_array {
            let (nline, nchar) = ((x.0 as i32 + dline) as usize, (x.1 as i32 + dchar) as usize);
            let Some(&b) = input_array
                .get(nline as usize)
                .and_then(|l| l.get(nchar as usize))
            else {
                return acc;
            };
            temp_vec.push(b.1);
        }
        let (first_cross, second_cross) = temp_vec.split_at(2);
        if first_cross.contains(&'M')
            && first_cross.contains(&'S')
            && second_cross.contains(&'M')
            && second_cross.contains(&'S')
        {
            return acc + 1;
        }
        acc
    })
}

pub fn part1() -> u32 {
    do_part1(INPUT_STRING)
}

pub fn part2() -> u32 {
    do_part2(INPUT_STRING)
}

// Check recursively
fn check_neighbors(
    input: &Vec<Vec<(usize, char)>>,
    idx: (usize, usize),
    check_vec: Vec<(i32, i32)>,
    check_char: char,
) -> u32 {
    let mut num = 0;
    for (dline, dchar) in check_vec {
        let (nline, nchar) = (
            (idx.0 as i32 + dline) as usize,
            (idx.1 as i32 + dchar) as usize,
        );
        let Some(&b) = input
            .get(nline as usize)
            .and_then(|l| l.get(nchar as usize))
        else {
            continue;
        };

        if b.1 == check_char {
            num += match check_char {
                'M' => check_neighbors(
                    input,
                    (nline as usize, nchar as usize),
                    vec![(dline, dchar)],
                    'A',
                ),
                'A' => check_neighbors(
                    input,
                    (nline as usize, nchar as usize),
                    vec![(dline, dchar)],
                    'S',
                ),
                'S' => 1,
                _ => 0,
            }
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX"};

    #[test]
    fn test_part1() {
        let res = do_part1(TEST);
        assert_eq!(18, res);
    }

    #[test]
    fn test_part2() {
        let res = do_part2(TEST);
        assert_eq!(9, res);
    }
}
