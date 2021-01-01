use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn get_loop_size(pub_key: u64, subject_number: u64) -> u64 {
    let mut iters = 0;
    let mut x = 1;
    loop {
        iters += 1;
        x *= subject_number;
        x %= 20201227;
        if x == pub_key {
            break;
        }
    }
    iters
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let card_pub_key = input.lines().nth(0).unwrap().parse::<u64>().unwrap();
    let door_pub_key = input.lines().nth(1).unwrap().parse::<u64>().unwrap();

    let card_loop_size = get_loop_size(card_pub_key, 7);
    let door_loop_size = get_loop_size(door_pub_key, 7);

    let mut encryption_key = 1;
    for _ in 0..card_loop_size {
        encryption_key *= door_pub_key;
        encryption_key %= 20201227;
    }
    println!("Part 1: {}", encryption_key);

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    Ok(())
}
