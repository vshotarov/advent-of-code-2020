use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut rules = parse_rules(&input);
    let messages = input.split("\n\n").nth(1).unwrap()
        .lines().collect::<Vec<&str>>();
    let mut invalid_messages: HashMap<String, Vec<String>> = HashMap::new();

    println!("Part 1: {}", messages.iter()
        .filter(|message| matches_rule(message, "0", &mut invalid_messages, &rules)).count());

    *rules.get_mut("8").unwrap() = vec![vec!["42".to_string()], vec!["42".to_string(), "8".to_string()]];
    *rules.get_mut("11").unwrap() = vec![vec!["42".to_string(), "31".to_string()], vec!["42".to_string(), "11".to_string(), "31".to_string()]];
    invalid_messages.clear();

    println!("Part 2: {}", messages.iter()
        .filter(|message| matches_rule(message, "0", &mut invalid_messages, &rules)).count());

    Ok(())
}

fn parse_rules(input: &str) -> HashMap<&str, Vec<Vec<String>>> {
    let mut rules: HashMap<&str, Vec<Vec<String>>> = HashMap::new();
    for line in input.split("\n\n").nth(0).unwrap().lines() {
        let left = line.split(": ").nth(0).unwrap();
        let right = line.split(": ").nth(1).unwrap().split(" | ")
            .map(|sequence| sequence.split(" ")
                .map(|x| x.replace("\"","")).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();
        rules.insert(left, right);
    }
    rules
}

fn matches_rule_sequence(message: &str, sequence: &[String],
invalid_messages: &mut HashMap<String, Vec<String>>,
rules: &HashMap<&str, Vec<Vec<String>>>) -> bool {
    if message.len() < sequence.len() {
        return false;
    }
    if sequence.len() == 0 {
        if message.len() == 0 {
            return true;
        }
        return false;
    }
    for i in 1..(message.len() + 1) {
        let prefix = &message[..i];
        let first_rule = &*sequence[0];
        if matches_rule(prefix, first_rule, invalid_messages, rules) {
            if matches_rule_sequence(&message[i..], &sequence[1..], invalid_messages, rules) {
                return true;
            }
        }
    }
    false
}

fn matches_rule(message: &str, rule_id: &str,
invalid_messages: &mut HashMap<String, Vec<String>>,
rules: &HashMap<&str, Vec<Vec<String>>>) -> bool {
    if invalid_messages.contains_key(message) {
        if invalid_messages[message].contains(&rule_id.to_string()) {
            return false;
        }
    }
    for sequence in &rules[rule_id] {
        if sequence.contains(&message.to_string()) {
            return true;
        }
        if ["a","b"].contains(&&*sequence[0]) {
            if invalid_messages.contains_key(message) {
                invalid_messages.get_mut(message).unwrap().push(rule_id.to_string());
            }
            else {
                invalid_messages.insert(message.to_string(), vec![rule_id.to_string()]);
            }
            return false;
        }
        if matches_rule_sequence(message, sequence, invalid_messages, rules) {
            return true;
        }
    }
    if invalid_messages.contains_key(message) {
        invalid_messages.get_mut(message).unwrap().push(rule_id.to_string());
    }
    else {
        invalid_messages.insert(message.to_string(), vec![rule_id.to_string()]);
    }
    false
}
