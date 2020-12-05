use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let taken_seats = solve_part1(&input)?;
    solve_part2(taken_seats)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<Vec<u32>> {
    let mut highest_seat_id = 0;
    let mut taken_seats: Vec<u32> = Vec::new();

    for line in input.lines() {
        let seat_id: u32 = line.chars().rev().enumerate()
            .fold(0, |acc, (i, letter)| match letter {
                'F' | 'L' => acc,
                'B' | 'R' => acc + (2 as u32).pow(i as u32),
                _ => acc
        });
        taken_seats.push(seat_id);
        if seat_id > highest_seat_id { highest_seat_id = seat_id; }
    }

    println!("Part 1: Highest seat ID {}", highest_seat_id);

    Ok(taken_seats)
}

fn solve_part2(taken_seats: Vec<u32>) -> std::io::Result<()> {
    for i in 1..(127 * 8 + 7) as u32 {
        if taken_seats.contains(&i) {
            continue;
        }

        if taken_seats.contains(&(i-1)) && taken_seats.contains(&(i+1)) {
            println!("Part 2: Your seat is {}", i);
            break;
        }
    }

    Ok(())
}
