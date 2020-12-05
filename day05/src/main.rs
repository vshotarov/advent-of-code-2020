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
        let mut row_left_pointer = 0;
        let mut row_right_pointer = 127;
        let mut col_left_pointer = 0;
        let mut col_right_pointer = 7;

        for letter in line.chars() {
            match letter {
                'F' => row_right_pointer -= (row_right_pointer - row_left_pointer) / 2 + 1,
                'B' => row_left_pointer += (row_right_pointer - row_left_pointer) / 2 + 1,
                'L' => col_right_pointer -= (col_right_pointer - col_left_pointer) / 2 + 1,
                'R' => col_left_pointer += (col_right_pointer - col_left_pointer) / 2 + 1,
                _ => println!("invalid char {}", letter),
            }
        }

        let seat_id = row_left_pointer * 8 + col_left_pointer;

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
