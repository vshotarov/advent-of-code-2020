use std::io::{self, Read};

#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32
}

impl Vec2 {
    fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn _rotate(&mut self, num_turns: i32, direction: i32) {
        for _ in 0..num_turns {
            let old_x = self.x;
            self.x = -self.y * direction;
            self.y = old_x * direction;
        }
    }

    fn rotate_left(&mut self, num_turns: i32) {
        self._rotate(num_turns, 1);
    }

    fn rotate_right(&mut self, num_turns: i32) {
        self._rotate(num_turns, -1);
    }
}

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let mut position: Vec2 = Vec2{ x:0, y:0 };
    let mut direction: Vec2 = Vec2{ x:1, y:0 };
    for instruction in input.lines().map(
            |x| (x.chars().nth(0).unwrap(),
                 x[1..].parse::<i32>().unwrap())) {
        match instruction.0 {
            'F' => position.translate(direction.x * instruction.1, direction.y * instruction.1),
            'E' => position.translate(instruction.1, 0),
            'W' => position.translate(-instruction.1, 0),
            'N' => position.translate(0, instruction.1),
            'S' => position.translate(0, -instruction.1),
            'L' => direction.rotate_left((instruction.1 % 360) / 90),
            'R' => direction.rotate_right((instruction.1 % 360) / 90),
            _ => println!("unrecognized instruction {}", instruction.0),
        }
    }

    println!("Part 1: {}", position.x.abs() + position.y.abs());

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let mut position: Vec2 = Vec2{ x:0, y:0 };
    let mut direction: Vec2 = Vec2{ x:10, y:1 };
    for instruction in input.lines().map(
            |x| (x.chars().nth(0).unwrap(),
                 x[1..].parse::<i32>().unwrap())) {
        match instruction.0 {
            'F' => position.translate(direction.x * instruction.1, direction.y * instruction.1),
            'E' => direction.translate(instruction.1, 0),
            'W' => direction.translate(-instruction.1, 0),
            'N' => direction.translate(0, instruction.1),
            'S' => direction.translate(0, -instruction.1),
            'L' => direction.rotate_left((instruction.1 % 360) / 90),
            'R' => direction.rotate_right((instruction.1 % 360) / 90),
            _ => println!("unrecognized instruction {}", instruction.0),
        }
    }

    println!("Part 2: {}", position.x.abs() + position.y.abs());

    Ok(())
}
