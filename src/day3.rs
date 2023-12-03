use std::collections::HashMap;

fn part_one(input: &str) -> i64 {
    let line_length = input.find('\n').unwrap() + 1;
    let number_of_lines = input.lines().count();

    let num_re = regex::Regex::new(r"\d+").unwrap();
    let nums: Vec<_> = num_re.find_iter(input).collect();

    let symbol_re = regex::Regex::new(r"[^.\d\n]").unwrap();
    let symbols: Vec<_> = symbol_re.find_iter(input).collect();

    symbols
        .iter()
        .flat_map(|symbol| {
            let symbol_pos = symbol.start();
            let neighbors = get_neighbors(symbol_pos, line_length, number_of_lines);

            nums.iter()
                .filter(move |num| {
                    neighbors
                        .iter()
                        .any(|&ne| ne >= num.start() && ne < num.end())
                })
                .filter_map(|n| {
                    n.as_str().parse::<i64>().ok().map(|num| ((n.start(), n.end()), num))
                })
        })
        .collect::<HashMap<_, _>>()
        .values()
        .sum()
}

fn part_two(input: &str) -> i64 {
    let line_length = input.find('\n').unwrap() + 1;
    let number_of_lines = input.lines().count();

    let num_re = regex::Regex::new(r"\d+").unwrap();
    let nums: Vec<_> = num_re.find_iter(input).collect();

    let symbol_re = regex::Regex::new(r"\*").unwrap();
    let symbols: Vec<_> = symbol_re.find_iter(input).collect();

    symbols
        .iter()
        .map(|symbol| {
            let symbol_pos = symbol.start();
            let neighbors = get_neighbors(symbol_pos, line_length, number_of_lines);

            nums.iter()
                .filter(|n| neighbors.iter().any(|&ne| ne >= n.start() && ne < n.end()))
                .map(|a| a.as_str().parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|grf| grf.len() == 2)
        .map(|grf| grf[0] * grf[1])
        .sum()
}

fn get_neighbors(symbol_pos: usize, line_length: usize, number_of_lines: usize) -> Vec<usize> {
    let row = symbol_pos / line_length;
    let col = symbol_pos % line_length;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    directions
        .iter()
        .filter_map(|&(dr, dc)| {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0
                && new_col >= 0
                && (new_row as usize) < number_of_lines
                && (new_col as usize) < line_length
            {
                Some((new_row as usize) * line_length + (new_col as usize))
            } else {
                None
            }
        })
        .collect()
}

pub fn solve(input: &str) -> (i64, i64) {
    (part_one(input), part_two(input))
}
