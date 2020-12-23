use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let mut sum = 0;
    for line in input.lines() {
        sum += eval_expr(line);
    }

    println!("Part 1: {}", sum);

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let mut sum = 0;
    for line in input.lines() {
        sum += eval_expr(&*bracketize_addition(line));
    }

    println!("Part 2: {}", sum);
    
    Ok(())
}

fn eval_expr(input: &str) -> u64 {
    let mut level = 0;
    let mut buffers: Vec<u64> = vec![0];
    let mut operations: Vec<&str> = vec!["+"];
    for op in input.split(" ") {
        let mut op_copy = op.clone();
        while op_copy.chars().nth(0).unwrap() == '(' {
            op_copy = &op_copy[1..];
            level += 1;
            buffers.push(0);
            operations.push("+");
        }
        match op_copy {
            "+" | "*" => operations.push(op_copy),
            mut x => {
                let mut buffers_to_collapse = 0;
                while x.chars().last().unwrap() == ')' {
                    buffers_to_collapse += 1;
                    x = &x[..(x.len()-1)];
                }
                if x.len() > 0 {
                    match operations.pop().unwrap() {
                        "*" => buffers[level] *= x.parse::<u64>().unwrap(),
                        _ => buffers[level] += x.parse::<u64>().unwrap(),
                    }
                }
                for _ in 0..buffers_to_collapse {
                    level -= 1;
                    buffers[level] = match operations.pop().unwrap() {
                        "*" => buffers.pop().unwrap() * buffers[level],
                        _ => buffers.pop().unwrap() + buffers[level],
                    }
                }
            }
        }
    }
    buffers[0]
}

fn bracketize_addition(input: &str) -> String {
    let num_additions = input.chars().fold(0, |acc,x| acc + match x {
        '+' => 1,
        _ => 0});
    let mut num_processed_additions = 0;
    let mut bracketized: String = input.to_string();

    while num_processed_additions < num_additions {
        let mut input_copy = bracketized.to_string();
        let mut num_seen_additions = 0;

        for (i,x) in bracketized.chars().enumerate() {
            if x == '+' {
                if num_seen_additions < num_processed_additions {
                    num_seen_additions += 1;
                    continue;
                }
                // find left side
                let mut pointer = i;
                let mut level = 0;
                let mut look_for_space = false;
                while pointer > 0 {
                    pointer -= 1;
                    match &bracketized[pointer..(pointer+1)] {
                        ")" => level += 1,
                        "(" => { level -= 1; if level == 0 { break; }}
                        " " => { if look_for_space { break; }}
                        "+" => { if level == 0 { break; }},
                        _ => { if level == 0 { look_for_space = true; }}
                    }
                }
                if pointer != 0 {
                    input_copy.insert_str(pointer+1, "(");
                }
                else {
                    input_copy.insert_str(pointer, "(");
                }

                // find right side
                pointer = i;
                level = 0;
                look_for_space = false;
                while pointer < bracketized.len() - 1 {
                    pointer += 1;
                    match &bracketized[pointer..(pointer+1)] {
                        "(" => level += 1,
                        ")" => { level -= 1; if level == 0 { break; }}
                        " " => { if look_for_space { break; }}
                        "+" => { if level == 0 { break; }},
                        _ => { if level == 0 { look_for_space = true; }}
                    }
                }
                if pointer == bracketized.len() - 1 {
                    input_copy.push_str(")");
                }
                else {
                    input_copy.insert_str(pointer + 1, ")");

                }
                break;
            }
        }
        num_processed_additions += 1;
        bracketized = input_copy.to_string();
    }
    bracketized
}
