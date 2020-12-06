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
    println!("Part 1: {}", input
        .split("\n\n")
        .fold(
            0, |acc, x| acc + HashSet::<char>::from(
                x.replace("\n","").chars().collect()).len()));

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

#[allow(dead_code)]
fn solve_part2_with_sets(input: &str) -> std::io::Result<()> {
    // Significantly slower than the naive part2 solution
    // This one takes around 47ms on my laptop, while the other one
    // takes around 18ms
    let mut yes_questions = 0;
    for group in input.split("\n\n") {
        let mut sets_iter = group
            .lines()
            .map(
                |x| HashSet::<char>::from(x.chars().collect()));

        let mut first_set = sets_iter.nth(0).unwrap();

        for _set in sets_iter {
            first_set = &first_set & &_set; // intersection
        }

        yes_questions += first_set.len();
    }
    println!("Part 2: {}", yes_questions);

    Ok(())
}
