use std::collections::HashMap;

fn part_one(input: &str) -> i64 {
    let split_re = regex::Regex::new(r"[:|]").unwrap();
    let number_re = regex::Regex::new(r"\d+").unwrap();

    input
        .lines()
        .filter_map(|line| {
            let stuff: Vec<_> = split_re.split(line).collect();
            let winning_numbers: Vec<_> =
                number_re.find_iter(stuff[1]).map(|n| n.as_str()).collect();
            let our_numbers: Vec<_> = number_re.find_iter(stuff[2]).map(|n| n.as_str()).collect();

            let matches = our_numbers
                .iter()
                .filter(|num| winning_numbers.contains(&num))
                .count() as i64;

            match matches {
                0 => None,
                _ => Some(2_i64.pow((matches - 1) as u32)),
            }
        })
        .sum()
}

fn part_two(input: &str) -> i64 {
    let split_re = regex::Regex::new(r"[:|]").unwrap();
    let number_re = regex::Regex::new(r"\d+").unwrap();
    let mut card_counter = CardCounter::new();
    let mut card_matches = HashMap::<i64, i64>::new();

    for line in input.lines() {
        let stuff: Vec<_> = split_re.split(line).collect();

        let card = number_re
            .find_iter(&stuff[0])
            .map(|n| n.as_str().parse().unwrap())
            .next()
            .unwrap();
        let winning_numbers: Vec<_> = number_re
            .find_iter(&stuff[1])
            .map(|n| n.as_str())
            .collect::<Vec<_>>();
        let our_numbers: Vec<_> = number_re.find_iter(&stuff[2]).map(|n| n.as_str()).collect();

        let matches: Vec<_> = our_numbers
            .iter()
            .filter(|num| winning_numbers.contains(num))
            .map(|num| num.parse::<i64>().unwrap())
            .collect();

        card_matches.insert(card, matches.len() as i64);
    }

    card_matches
        .keys()
        .map(|card| card_counter.count(*card, &card_matches))
        .sum()
}

pub fn solve(input: &str) -> (i64, i64) {
    (part_one(input), part_two(input))
}

struct CardCounter {
    cache: HashMap<i64, i64>,
}

impl CardCounter {
    fn new() -> Self {
        CardCounter {
            cache: HashMap::new(),
        }
    }

    fn count(&mut self, start: i64, matches: &HashMap<i64, i64>) -> i64 {
        if let Some(&cached_result) = self.cache.get(&start) {
            return cached_result;
        }

        let match_count = matches[&start];
        let card_max = *matches.keys().max().unwrap() as i64;
        let start_range = std::cmp::min(start + 1, card_max);
        let end_range = std::cmp::min(start + match_count, card_max);
        let new_cards = start_range..=end_range;

        let result = match match_count {
            0 => 1,
            _ => 1 + new_cards.map(|c| self.count(c, matches)).sum::<i64>(),
        };

        self.cache.insert(start, result);

        result
    }
}
