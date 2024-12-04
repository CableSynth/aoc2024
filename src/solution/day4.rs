use itertools::Itertools;

const INPUT_STRING: &str = include_str!("../../input/day_4");

fn parse(input: &str) -> Vec<Vec<(usize, char)>> {
    input.lines().map(|l| l.char_indices().collect_vec()).collect_vec()
}

pub fn part1(input: &str) -> u32 {
    let input_array = parse(input);
    // filter for indcies that hold an x
    let height = input_array[0].len();
    let check_array = [(-1, -1), (-1, 0), (0, -1), (0, 1), (1, -1), (1, 1)];
    let x_indicies = input_array
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            c.iter()
                .filter_map(|(i, ch)| if *ch == 'X' { Some((idx, *i)) } else { None })
        })
        .flatten()
        .collect_vec();
    x_indicies.iter().fold(0, |acc, x| {});
    // Check the neighbors from the indicies above
    0
}

// Check recursively
fn check_neighbors(input: Vec<Vec<(usize, char)>>, idx: (usize, usize), check_vec: [(i32, i32); 6], check_char: char) -> u32 {
    let valid_pos = input.iter().filter_map(||)
    match check_char {
        'M' => {}
        'A' => {}
        'S' => {}
        _ => 0
    }
}
