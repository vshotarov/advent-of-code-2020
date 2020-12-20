use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let parsed_rules: Vec<_> = input.split("\n\n").nth(0).unwrap().lines().map(
        |line| move |x: u32| {
            let rule1: Vec<u32> = line.split(": ").nth(1).unwrap()
                .split(" or ").nth(0).unwrap().split("-")
                .map(|num| num.parse::<u32>().unwrap()).collect();
            let rule2: Vec<u32> = line.split(": ").nth(1).unwrap()
                .split(" or ").nth(1).unwrap().split("-")
                .map(|num| num.parse::<u32>().unwrap()).collect();
            ((x >= rule1[0] && x <= rule1[1]) || (x >= rule2[0] && x <= rule2[1])) as bool
        }).collect();

    let nearby_tickets: Vec<Vec<u32>> = input.split("\n\n").nth(2).unwrap().lines()
        .skip(1).map(|line| line.split(",").map(
            |x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect();

    let error_rate = nearby_tickets.iter().fold(0,
        |acc, ticket| acc + ticket.iter().fold(0,
            |ticket_acc, x| ticket_acc + x * (!parsed_rules.iter().fold(false,
                |rule_acc, rule| rule_acc || rule(*x))) as u32));

    println!("Part 1: {}", error_rate);

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let rules = input.split("\n\n").nth(0).unwrap();
    let parsed_rules: Vec<_> = rules.lines().map(
        |line| move |x: u32| {
            let rule1: Vec<u32> = line.split(": ").nth(1).unwrap()
                .split(" or ").nth(0).unwrap().split("-")
                .map(|num| num.parse::<u32>().unwrap()).collect();
            let rule2: Vec<u32> = line.split(": ").nth(1).unwrap()
                .split(" or ").nth(1).unwrap().split("-")
                .map(|num| num.parse::<u32>().unwrap()).collect();
            ((x >= rule1[0] && x <= rule1[1]) || (x >= rule2[0] && x <= rule2[1])) as bool
        }).collect();

    let nearby_tickets: Vec<Vec<u32>> = input.split("\n\n").nth(2).unwrap().lines()
        .skip(1).map(|line| line.split(",").map(
            |x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect();

    let valid_tickets: Vec<&Vec<u32>> = nearby_tickets.iter().filter(
        |ticket| ticket.iter().fold(true,
            |ticket_acc, x| ticket_acc && parsed_rules.iter().fold(false,
                |rule_acc, rule| rule_acc || rule(*x)))).collect();

    let num_rules = parsed_rules.len();

    let mut valid_positions: Vec<Vec<u32>> = Vec::new();
    for rule in &parsed_rules {
        let mut rule_possible_positions: Vec<bool> = vec![true; num_rules];
        for ticket in &valid_tickets {
            for position in 0..num_rules {
                if !rule_possible_positions[position] {
                    continue;
                }
                rule_possible_positions[position] = rule_possible_positions[position] && rule(ticket[position]);
            }
        }
        let mut rule_valid_positions: Vec<u32> = Vec::new();
        for (i, position) in rule_possible_positions.iter().enumerate() {
            if *position {
                rule_valid_positions.push(i as u32);
            }
        }
        valid_positions.push(rule_valid_positions);
    }

    let mut rule_positions: Vec<u32> = vec![1000; num_rules];
    loop {
        for i in 0..num_rules {
            if valid_positions[i].len() != 1 {
                continue;
            }
            rule_positions[i] = valid_positions[i].pop().unwrap();
            for j in 0..num_rules {
                match valid_positions[j].iter().position(|x| *x == rule_positions[i]) {
                    Some(x) => { valid_positions[j].remove(x); () },
                    None => ()
                }
            }
        }
        if !rule_positions.contains(&1000) {
            break;
        }
    }
    let my_ticket: Vec<u64> = input.split("\n\n").nth(1).unwrap().lines().nth(1).unwrap()
        .split(",").map(|x| x.parse::<u64>().unwrap()).collect();
    let departure_fields: Vec<usize> = rules.lines().enumerate()
        .filter(|(i,x)| x.contains("departure")).map(|(i,x)| i).collect();

    println!("Part 2: {}", departure_fields.iter().fold(1,
            |acc,x| acc * my_ticket[rule_positions[*x] as usize]));

    Ok(())
}
