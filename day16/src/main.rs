use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let rules = input.split("\n\n").nth(0).unwrap();
    let nearby_tickets = input.split("\n\n").nth(2).unwrap();

    let parsed_rules = rules.lines().map(
        |line| line.split(": ").nth(1).unwrap().split(" or ").map(
            |constrain| {
                let split_constrain: Vec<u32> = constrain.split("-").map(|x| x.parse::<u32>().unwrap()).collect();
                (split_constrain[0], split_constrain[1])
            }).collect::<Vec<(u32,u32)>>()).collect::<Vec<Vec<(u32,u32)>>>();

    let error_rate = nearby_tickets.lines().fold(0,
        |acc, x| {
            if x.contains("nearby") {
                return 0;
            }
            acc + x.split(",").fold(0,
                |_acc, _x| {
                    let parsed_x = _x.parse::<u32>().unwrap();
                    let mut valid = false;
                    for rule in &parsed_rules {
                        let mut local_valid = false;
                        for constrain in rule {
                            if parsed_x >= constrain.0 && parsed_x <= constrain.1 {
                                local_valid = true;
                                break;
                            }
                        }
                        if local_valid {
                            valid = true;
                        }
                        else {
                        }
                    }
                    if !valid {
                        _acc + parsed_x
                    }
                    else {
                        _acc + 0
                    }
                })
        });

    println!("Part 1: {}", error_rate);

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let rules = input.split("\n\n").nth(0).unwrap();
    let my_ticket = input.split("\n\n").nth(1).unwrap();
    let nearby_tickets = input.split("\n\n").nth(2).unwrap();

    let parsed_rules = rules.lines().map(
        |line| line.split(": ").nth(1).unwrap().split(" or ").map(
            |constrain| {
                let split_constrain: Vec<u32> = constrain.split("-").map(|x| x.parse::<u32>().unwrap()).collect();
                (split_constrain[0], split_constrain[1])
            }).collect::<Vec<(u32,u32)>>()).collect::<Vec<Vec<(u32,u32)>>>();

    let valid_tickets = nearby_tickets.lines().filter(
        |x| {
            if x.contains("nearby") {
                return false;
            }
            x.split(",").fold(0,
                |_acc, _x| {
                    let parsed_x = _x.parse::<u32>().unwrap();
                    let mut valid = false;
                    for rule in &parsed_rules {
                        let mut local_valid = false;
                        for constrain in rule {
                            if parsed_x >= constrain.0 && parsed_x <= constrain.1 {
                                local_valid = true;
                                break;
                            }
                        }
                        if local_valid {
                            valid = true;
                        }
                        else {
                        }
                    }
                    if !valid {
                        _acc + parsed_x
                    }
                    else {
                        _acc + 0
                    }
                }) == 0
        }).collect::<Vec<&str>>();

    let num_fields = parsed_rules.len();
    let mut all_valid_positions: Vec<Vec<Vec<bool>>> = Vec::new();
    for rule in &parsed_rules {
        let mut rule_valid_positions: Vec<Vec<bool>> = Vec::new();
        for ticket in &valid_tickets {
            let mut valid_positions: Vec<bool> = vec![true; num_fields];
            for (i, value) in ticket.split(",").enumerate() {
                let parsed_value = value.parse::<u32>().unwrap();
                if !((parsed_value >= rule[0].0 && parsed_value <= rule[0].1) ||
                    (parsed_value >= rule[1].0 && parsed_value <= rule[1].1)) {
                    valid_positions[i] = false;
                }
            }
            rule_valid_positions.push(valid_positions);
        }
        all_valid_positions.push(rule_valid_positions);
    }

    let num_rules = parsed_rules.len();
    let mut rule_positions: Vec<i32> = vec![-1; num_rules];
    let mut taken_positions: Vec<usize> = Vec::new();

    loop {
        for (i, rule_valid_positions) in all_valid_positions.iter().enumerate() {
            if rule_positions[i] != -1 {
                continue;
            }
            let mut possible_position = -1;
            'outer: for j in 0..num_rules {
                if taken_positions.contains(&j) {
                    continue;
                }
                for ticket in rule_valid_positions {
                    if !ticket[j] {
                        continue 'outer;
                    }
                }
                if possible_position == -1 {
                    possible_position = j as i8;
                }
                else {
                    possible_position = -1;
                    break;
                }
            }
            if possible_position != -1 {
                rule_positions[i] = possible_position as i32;
                taken_positions.push(possible_position as usize);
            }
        }

        if taken_positions.len() == num_rules - 1{
            let mut to_assign = 0;
            let mut assign_to = 0;
            for (j, rule) in rule_positions.iter().enumerate() {
                if *rule == -1 {
                    assign_to = j;
                    for position in 0..num_rules {
                        if !taken_positions.contains(&position) {
                            to_assign = position;
                            break;
                        }
                    }
                }
            }
            rule_positions[assign_to] = to_assign as i32;
            break;
        }
    }

    let mut departure_fields_product = 1;
    for (i, rule) in rules.lines().enumerate() {
        if rule.contains("departure") {
            departure_fields_product *= my_ticket.lines().nth(1).unwrap().split(",").nth(
                rule_positions[i] as usize).unwrap().parse::<u64>().unwrap();
        }
    }

    println!("Part 2: {}", departure_fields_product);

    Ok(())
}
