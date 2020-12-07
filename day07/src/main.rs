use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct BagCount {
    bag: String,
    count: u32,
}

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.replace(".","").lines() {
        let rule: Vec<&str> = line.split(" bags contain ").collect();
        let container: &str = rule[0];
        let contents: &str = rule[1];

        for bag in contents.split(", ") {
            if bag == "no other bags" { continue; }
            let bag_type = bag[2..].split(" bag").nth(0).unwrap();
            if !rules.contains_key(bag_type) {
                rules.insert(bag_type.to_string(), Vec::new());
            }
            rules.get_mut(bag_type).unwrap().push(container.to_string());
        }
    }

    let mut num_options = 0;

    let mut explore_list = rules["shiny gold"].clone();
    let mut seen: Vec<String> = Vec::new();
    while explore_list.len() > 0 {
        num_options += 1;
        let bag = explore_list.pop().unwrap();
        seen.push(bag.clone());

        if rules.contains_key(&bag) {
            for container in &rules[&bag] {
                if !seen.contains(container) {
                    explore_list.push(container.clone());
                }
            }
        }
    }

    println!("Part 1: {}", num_options);

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.replace(".","").lines() {
        let rule: Vec<&str> = line.split(" bags contain ").collect();
        let container: &str = rule[0];
        let contents: &str = rule[1];

        if contents != "no other bags" {
            rules.insert(container.to_string(), contents.split(", ").
                map(|x| x.to_string()).collect());
        }
    }

    let mut total_bags = 0;
    let mut explore_list: VecDeque<BagCount> = VecDeque::new();
    explore_list.push_back(BagCount {bag : "shiny gold".to_string(), count : 1});
    while explore_list.len() > 0 {
        let bag_count = explore_list.pop_front().unwrap();

        total_bags += bag_count.count;

        if !rules.contains_key(&bag_count.bag) {
            continue;
        }

        for contained in &rules[&bag_count.bag] {
            explore_list.push_back(BagCount {
                bag: contained[2..].split(" bag").nth(0).unwrap().to_string(),
                count : contained[..1].parse::<u32>().unwrap() * bag_count.count});
        }
    }

    println!("Part 2: {}", total_bags - 1);

    Ok(())
}
