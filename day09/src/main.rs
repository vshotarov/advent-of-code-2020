use std::io::{self, Read};
use std::collections::VecDeque;

static BUFFER_SIZE: usize = 25;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let first_invalid_number = solve_part1(&input)?;
    solve_part2(&input, first_invalid_number)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<i64> {
    let mut buffer: VecDeque<i64> = VecDeque::new();

    for line in input.lines() {
        let buffer_len = buffer.len();
        let number = line.parse::<i64>().unwrap();

        if buffer_len == BUFFER_SIZE {
            let mut found_two_sum = false;

            for n in &buffer {
                if buffer.contains(&(number - n)) {
                    found_two_sum = true;
                    break;
                }
            }

            if !found_two_sum {
                println!("Part 1: {}", number);
                return Ok(number);
            }
        }

        buffer.push_back(number);
        if buffer_len > BUFFER_SIZE - 1 {
            buffer.pop_front();
        }
    }

    Ok(-1)
}

fn solve_part2(input: &str, sum: i64) -> std::io::Result<()> {
    let numbers: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    let mut left_pointer = 0;
    let mut right_pointer = 1;
    let mut running_sum = numbers[0] + numbers[1];

    loop {
        if running_sum == sum {
            println!("Part 2: {}",
                numbers[left_pointer..right_pointer+1].iter().min().unwrap() +
                numbers[left_pointer..right_pointer+1].iter().max().unwrap());
            break;
        }
        else if running_sum < sum {
            right_pointer += 1;
            running_sum += numbers[right_pointer];
        }
        else {
            left_pointer += 1;
            right_pointer = left_pointer + 1;
            running_sum = numbers[left_pointer] + numbers[right_pointer];
        }
    }

    Ok(())
}
