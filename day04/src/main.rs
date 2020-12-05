use std::io::{self, Read};
use regex::Regex;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let mut num_valid_passports = 0;
    for passport in input.split("\n\n") {
        let num_fields = passport.replace("\n"," ").split_whitespace().count();
        let has_cid = passport.contains("cid:");

        if num_fields == 8 || (num_fields == 7 && !has_cid) {
            num_valid_passports += 1;
        }
    }

    println!("Part 1: Num valid passports {}", num_valid_passports);

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let hcl_regex = Regex::new("#[a-f0-9]{6}").unwrap();
    let pid_regex = Regex::new("^[0-9]{9}$").unwrap();
    let hgt_regex = Regex::new("[0-9]*[cm|in]").unwrap();
    let mut num_valid_passports = 0;

    'outer: for passport in input.split("\n\n") {
        let mut num_fields = 0;

        for kv_pair in passport.replace("\n"," ").split_whitespace() {
            let kv_pair_split: Vec<&str> = kv_pair.split(":").collect();
            let (key, value) = (kv_pair_split[0], kv_pair_split[1]);

            if !["byr","iyr","eyr","hgt","hcl","ecl","pid","cid"].contains(&key) {
                continue 'outer;
            }

            num_fields += 1;

            if key == "byr" && !validate_year(value, 1920, 2002) { continue 'outer; }
            if key == "iyr" && !validate_year(value, 2010, 2020) { continue 'outer; }
            if key == "eyr" && !validate_year(value, 2020, 2030) { continue 'outer; }
            if key == "hgt" { 
                if !hgt_regex.is_match(value) { continue 'outer; }
                let unit = match value.contains("cm") {
                    true => "cm",
                    false => "in"};
                let height = value.replace(unit, "").parse::<u32>().unwrap_or(0);
                if (unit == "cm" && (height < 150 || height > 193)) ||
                    (unit == "in" && (height < 59 || height > 76)) {
                        continue 'outer;
                }
            }
            if key == "hcl" && !hcl_regex.is_match(value) { continue 'outer; }
            if key == "ecl" && !(["amb","blu","brn","gry","grn","hzl","oth"].contains(&value)) {
                continue 'outer;
            }
            if key == "pid" && !pid_regex.is_match(value) { continue 'outer; }
        }

        if num_fields == 8 || (num_fields == 7 && !passport.contains("cid")){
            num_valid_passports += 1;
        }
    }

    println!("Part 2: Num valid passports {}", num_valid_passports);

    Ok(())
}

fn validate_year(year: &str, min: u32, max: u32) -> bool {
    if year.len() != 4 {
        return false;
    }
    let year_as_number = year.parse::<u32>().unwrap_or(0);
    if year_as_number < min || year_as_number > max {
        return false;
    }
    true
}
