fn part_one(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let nums: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

        let first_digit = nums[0];
        let second_digit = nums[nums.len() - 1];

        let mut full_num_string = String::new();

        full_num_string.push(first_digit);
        full_num_string.push(second_digit);

        sum += full_num_string.parse::<i64>().expect("An integer");
    }

    return sum;
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
    let mut sum = 0;
    let digit_pattern = r"\d|one|two|three|four|five|six|seven|eight|nine";
    let re = regex::Regex::new(digit_pattern).unwrap();

    for line in input.lines() {
        let mut matches: Vec<i64> = Vec::new(); 

        let mut offset  = 0;
        while let Some(mat) = re.find(&line[offset..]) {
            offset += mat.start() + 1;
            matches.push(string_to_digit(mat.as_str()));
        }

        let first_digit = matches[0];
        let last_digit = if matches.len() > 1 {
            matches[matches.len() - 1]
        } else {
            matches[0]
        };
        
        let full_num_string = format!("{}{}", first_digit, last_digit);

        sum += full_num_string
            .parse::<i64>()
            .expect("Expected a valid integer");
    }

    sum
}

pub fn solve(input: &str) -> (i64, i64) {
    return (part_one(input), part_two(input));
}