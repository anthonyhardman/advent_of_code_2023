fn part_one(input: &str) -> i64 {
    input.lines().map(|line| {
        let nums: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

        let first_digit = nums.first().unwrap();
        let last_digit = nums.last().unwrap();

        format!("{first_digit}{last_digit}").parse::<i64>().unwrap()
    }).sum()
}

fn string_to_digit(digit_string: &str) -> i64 {
    match digit_string {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        single_digit => single_digit.parse::<i64>().unwrap()
    }
}

fn part_two(input: &str) -> i64 {
    let digit_pattern = r"\d|one|two|three|four|five|six|seven|eight|nine";
    let re = regex::Regex::new(digit_pattern).unwrap();
    
    input.lines().map(|line| {
        let mut matches: Vec<i64> = Vec::new(); 

        let mut offset  = 0;
        while let Some(mat) = re.find(&line[offset..]) {
            offset += mat.start() + 1;
            matches.push(string_to_digit(mat.as_str()));
        }

        let first_digit = matches.first().unwrap();
        let last_digit = matches.last().unwrap();

        format!("{first_digit}{last_digit}").parse::<i64>().unwrap()
    }).sum()
}

pub fn solve(input: &str) -> (i64, i64) {
    (part_one(input), part_two(input))
}