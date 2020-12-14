use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let target_time = input.lines().nth(0).unwrap()
        .parse::<u32>().unwrap();
    let mut best_bus = 0;
    let mut best_time_to_wait = 64000;

    for bus in input.lines().nth(1).unwrap().replace("x,","")
            .split(",").map(|x| x.parse::<u32>().unwrap()) {
        if target_time % bus == 0 {
            best_bus = bus;
            best_time_to_wait = 0;
            break;
        }
        let time_to_wait = ((target_time / bus) + 1) * bus - target_time;
        if time_to_wait < best_time_to_wait {
            best_bus = bus;
            best_time_to_wait = time_to_wait;
        }
    }

    println!("Part 1: {}", best_bus * best_time_to_wait); 

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let mut buses = input.lines().nth(1).unwrap()
        .split(",").enumerate().map(|(i,x)|
            if x == "x" {
                (i as u64, 0)
            }
            else {
                (i as u64, x.parse::<u64>().unwrap())
            }).filter(|(_,x)| *x != 0 );

    let first_bus = buses.nth(0).unwrap().1;
    let mut timestamp = first_bus;
    let mut time_step = first_bus;

    for (time, bus) in buses {
        if time == 0 {
            continue;
        }

        loop {
            if (timestamp + time) % bus == 0 {
                break;
            }
            timestamp += time_step;
        }
        // The time_step becomes the LCM of the current time_step and the bus, as if our timestamp
        // is not dividable by any of those, it's not a solution
        time_step = time_step * bus;
    }

    println!("Part 2: {}", timestamp);

    Ok(())
}
