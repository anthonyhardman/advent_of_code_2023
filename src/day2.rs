fn part_one(input: &str) -> i64 {
    parse_game(input)
        .iter()
        .filter(|g| g.1.iter().all(|d| valid_game(*d)))
        .map(|g| g.0)
        .sum()
}

fn part_two(input: &str) -> i64 {
    parse_game(input)
        .iter()
        .map(|game| {
            let max_red = max_color("red", game);
            let max_green = max_color("green", game);
            let max_blue = max_color("blue", game);

            max_red * max_green * max_blue
        })
        .sum()
}

pub fn solve(input: &str) -> (i64, i64) {
    (part_one(input), part_two(input))
}

fn parse_game(input: &str) -> Vec<(i64, Vec<(&str, i64)>)> {
    let num_re = regex::Regex::new(r"\d+").unwrap();
    let color_re = regex::Regex::new(r"[a-zA-Z]+").unwrap();

    input
        .lines()
        .map(|line| {
            let nums: Vec<i64> = num_re
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect();

            let colors: Vec<&str> = color_re.find_iter(line).map(|m| m.as_str()).collect();
            let dice: Vec<(&str, i64)> = (1..nums.len()).map(|i| (colors[i], nums[i])).collect();

            (*nums.first().unwrap(), dice)
        })
        .collect()
}

fn valid_game(cube: (&str, i64)) -> bool {
    match cube.0 {
        "red" => cube.1 <= 12,
        "green" => cube.1 <= 13,
        "blue" => cube.1 <= 14,
        _ => false,
    }
}

fn max_color(color: &str, game: &(i64, Vec<(&str, i64)>)) -> i64 {
    game.1
        .iter()
        .filter(|c| c.0 == color)
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|c| c.1)
        .unwrap()
}
