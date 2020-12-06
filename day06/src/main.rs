use std::io::{self, Read};
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let mut yes_questions = 0;

    for group in input.split("\n\n") {
        let mut group_letters = group.replace("\n","");
        loop {
            let letter = match group_letters.chars().nth(0) {
                Some(a) => { yes_questions += 1; a },
                _ => break,
            };

            group_letters = group_letters.replace(letter, "");
        }
    }

    println!("Part 1: {}", yes_questions);

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let mut yes_questions = 0;

    for group in input.split("\n\n") {
        let num_people = group.lines().count() as i32;
        let mut group_letters = group.replace("\n","");
        loop {
            let letter = match group_letters.chars().nth(0) {
                Some(a) => a,
                _ => break,
            };

            let num_letters_before_replace = group_letters.len() as i32;
            group_letters = group_letters.replace(letter, "");
            if num_letters_before_replace - group_letters.len() as i32 == num_people {
                yes_questions += 1;
            }
        }
    }

    println!("Part 2: {}", yes_questions);

    Ok(())
}
