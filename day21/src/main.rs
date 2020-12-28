use std::io::{self, Read};
use std::collections::{HashMap, HashSet};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve(&input)?;

    Ok(())
}

fn solve(input: &str) -> std::io::Result<()> {
    let mut allergens: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    let mut ingredient_counts: HashMap<&str, u32> = HashMap::new();
    for line in input.lines() {
        let line_allergens = match line.split("(contains ").nth(1) {
            Some(x) => x.split(")").nth(0).unwrap().split(", ").collect::<Vec<&str>>(),
            None => continue,
        };
        let line_ingredients = line.split(" (contains").nth(0).unwrap()
            .split(" ").collect::<Vec<&str>>();
        for ingredient in &line_ingredients {
            if !ingredient_counts.contains_key(ingredient) {
                ingredient_counts.insert(ingredient, 0);
            }
            *ingredient_counts.get_mut(ingredient).unwrap() += 1;
            for allergen in &line_allergens {
                if !allergens.contains_key(allergen) {
                    allergens.insert(allergen, HashMap::new());
                }
                if !allergens[allergen].contains_key(ingredient) {
                    allergens.get_mut(allergen).unwrap().insert(ingredient, 0);
                }
                *allergens.get_mut(allergen).unwrap().get_mut(ingredient).unwrap() += 1;
            }
        }
    }

    let mut found_ingredients: HashSet<&str> = HashSet::new();
    let mut all_ingredients: HashSet<&str> = HashSet::new();
    for (_, ingredients) in &allergens {
        let max_amount = ingredients.values().max().unwrap();
        for (ingredient, amount) in ingredients {
            if amount == max_amount {
                found_ingredients.insert(ingredient);
            }
            all_ingredients.insert(ingredient);
        }
    }

    println!("Part 1: {}", all_ingredients.difference(&found_ingredients)
        .collect::<Vec<&&str>>().iter()
        .fold(0, |acc,x| acc + ingredient_counts[*x]));

    let mut found_allergens: HashMap<&str, &str> = HashMap::new();
    found_ingredients.clear();
    while found_allergens.len() != allergens.len() {
        'outer: for (allergen, ingredients) in &allergens {
            if found_allergens.contains_key(allergen) {
                continue;
            }
            let max_amount = ingredients.values().max().unwrap();
            let mut found_one = false;
            let mut found_ingredient: &str = "";
            for (ingredient, amount) in ingredients {
                if found_ingredients.contains(ingredient) {
                    continue;
                }
                if amount == max_amount {
                    if found_one {
                        continue 'outer;
                    }
                    else {
                        found_one = true;
                        found_ingredient = ingredient;
                    }
                }
            }
            found_allergens.insert(allergen, found_ingredient);
            found_ingredients.insert(found_ingredient);
        }
    }

    let mut allergen_keys = found_allergens.keys().collect::<Vec<&&str>>();
    allergen_keys.sort();
    let mut cdil: String = String::new(); // Canonical Dangerous Ingredient List 
    for allergen_key in allergen_keys {
        cdil.push_str(found_allergens[allergen_key]);
        cdil.push(',');
    }
    cdil.pop();
    
    println!("Part 2: {}", cdil);

    Ok(())
}
