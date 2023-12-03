fn parse_game(input: &str) -> Vec<(i64, Vec<(&str, i64)>)> {
    let num_re = regex::Regex::new(r"\d+").unwrap();
    let color_re = regex::Regex::new(r"[a-zA-Z]+").unwrap();
    let mut games = Vec::<(i64, Vec<(&str, i64)>)>::new();

    for line in input.lines() {
        let nums: Vec<i64> = num_re
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let colors: Vec<&str> = color_re.find_iter(line).map(|m| m.as_str()).collect();
        let game_id = nums[0];
        let dice: Vec<(&str, i64)> = (1..nums.len()).map(|i| (colors[i], nums[i])).collect();

        games.push((game_id, dice));
    }

    games
}

fn valid_game(cube: (&str, i64)) -> bool {
    match cube.0 {
        "red" => cube.1 <= 12,
        "green" => cube.1 <= 13,
        "blue" => cube.1 <= 14,
        _ => false,
    }
}

fn part_one(input: &str) -> i64 {
    let games = parse_game(input);
    let valid_games = games
        .iter()
        .filter(|g| g.1.iter().all(|d| valid_game(*d)))
        .map(|g| g.0)
        .collect::<Vec<_>>();

    return valid_games.iter().sum();
}

fn max_color(color: &str, game: &(i64, Vec<(&str, i64)>)) -> i64 {
    let max = game
        .1
        .iter()
        .filter(|c| c.0 == color)
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|c| c.1)
        .unwrap();

    max
}

fn part_two(input: &str) -> i64 {
    let games = parse_game(input);
    let mut sum = 0;

    for game in games.iter() {
        let max_red = max_color("red", game);
        let max_green = max_color("green", game);
        let max_blue = max_color("blue", game);
        sum += max_red * max_green * max_blue;
    }

    sum
}

pub fn solve(input: &str) -> (i64, i64) {
    return (part_one(input), part_two(input));
}