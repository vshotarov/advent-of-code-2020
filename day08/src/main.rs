use std::io::{self, Read};
use std::collections::HashSet;

struct RunInfo {
    acc: i32,
    terminated: bool
}

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let run_info = run_code(input, -1);

    println!("Part 1: {}", run_info.acc);

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let mut swap_n = 1;
    loop {
        let run_info = run_code(&input, swap_n);
        swap_n += 1;

        if run_info.terminated {
            println!("Part 2: {}", run_info.acc);
            break;
        }
    }

    Ok(())
}

fn run_code(code: &str, swap_nth: i32) -> RunInfo {
    let mut return_struct = RunInfo {acc: 0, terminated: false};
    let mut pointer: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    let lines: Vec<&str> = code.lines().collect();
    let mut swap_matches_seen = 0;
    let mut swap_line = -1;

    loop {
        if visited.contains(&pointer) {
            break;
        }
        visited.insert(pointer);

        let split_instruction: Vec<&str> = match lines.get(pointer as usize) {
            Some(line) => line.split_whitespace().collect(),
            None => { return_struct.terminated = true; break },
        };
        let mut instruction = split_instruction[0];
        let offset = split_instruction[1].parse::<i32>().unwrap();

        // If we are swapping operations, check if this is the swap_nth time
        // we see one of the swappable operations and if it is, mark the line for swapping
        if swap_nth != -1 && swap_line == -1 && ["jmp","nop"].contains(&instruction) {
            swap_matches_seen += 1;
            if swap_matches_seen == swap_nth {
                swap_line = pointer;
            }
        }

        // Perform the swap if we have to
        if pointer == swap_line {
            instruction = match instruction {
                "jmp" => "nop",
                "nop" => "jmp",
                _ => panic!("Unrecognized instr to swap {}", instruction)
            };
        }

        match instruction {
            "acc" => { return_struct.acc += offset; pointer += 1 },
            "jmp" => pointer += offset,
            "nop" => pointer += 1,
            _ => panic!("Unrecognized instruction {}", instruction)
        }

        if pointer < 0 {
            panic!("Pointer is {}, but it should never go below 0", pointer);
        }
    }

    return return_struct;
}
