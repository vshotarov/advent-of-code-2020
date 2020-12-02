use std::io::{self, Read};
use regex::Regex;

struct ParsedLine {
    left_rule: usize,
    right_rule: usize,
    letter: char,
    password: String
}

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn parse_line(line: &str, rule: &regex::Regex) -> ParsedLine {
    let captures: Vec<regex::Captures> = rule.captures_iter(line).collect();
    ParsedLine {
        left_rule: captures[0][1].parse::<usize>().unwrap(),
        right_rule: captures[0][2].parse::<usize>().unwrap(),
        letter: captures[0][3].chars().nth(0).unwrap(),
        password: captures[0][4].to_string()
    }
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let re = Regex::new(r"(\d*)-(\d*) ([a-z]): (\w*)").unwrap();

    let mut num_valid_passwords = 0;

    for line in input.lines() {
        let parsed = parse_line(line, &re);

        let mut num_occurrences = 0;
        for character in parsed.password.chars() {
            if character == parsed.letter { num_occurrences += 1; }
        }

        if num_occurrences >= parsed.left_rule && num_occurrences <= parsed.right_rule {
            num_valid_passwords += 1;
        }
    }

    println!("Part 1: Num valid passwords {}", num_valid_passwords);

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let re = Regex::new(r"(\d*)-(\d*) ([a-z]): (\w*)").unwrap();

    let mut num_valid_passwords = 0;

    for line in input.lines() {
        let parsed = parse_line(line, &re);

        let match_first = parsed.password.chars().nth(parsed.left_rule-1).unwrap() == parsed.letter;
        let match_second = parsed.password.chars().nth(parsed.right_rule-1).unwrap() == parsed.letter;

        if (match_first && match_second) == false && (match_first || match_second) == true {
            num_valid_passwords += 1;
        }
    }

    println!("Part 2: Num valid passwords {}", num_valid_passwords);

    Ok(())
}
