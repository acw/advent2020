use advent2020::errors::TopLevelError;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut mapping = HashMap::new();
    let mut all_words = HashSet::new();
    let mut every_word_listed = Vec::new();

    for line in contents.lines() {
        if let Some(lost_paren) = line.strip_suffix(')') {
            let mut parts = lost_paren.split(" (contains ");
            let words_iter = parts.next().unwrap().split(' ');
            let words: HashSet<&str> = words_iter.clone().collect();
            let allergens: Vec<&str> = parts.next().unwrap().split(", ").collect();

            every_word_listed.extend(words_iter);
            for allergen in allergens.iter() {
                match mapping.get_mut(allergen) {
                    None => {
                        mapping.insert(*allergen, words.clone());
                    }
                    Some(set) => {
                        set.retain(|x| words.contains(x));
                    }
                }
            }

            for word in words.iter() {
                all_words.insert(*word);
            }
        } else {
            return Err(TopLevelError::UnknownError);
        }
    }

    let mut possible_allergens = HashSet::new();

    for (key, val) in mapping.iter() {
        println!("{} ==> {:?}", key, val);
        possible_allergens = possible_allergens.union(val).cloned().collect();
    }

    let safe_foods: HashSet<&str> = all_words.difference(&possible_allergens).cloned().collect();
    println!(
        "count: {}",
        every_word_listed
            .iter()
            .filter(|x| safe_foods.contains(*x))
            .count()
    );

    while mapping.iter().any(|(_, r)| r.len() != 1) {
        let mut to_remove: HashSet<&str> = HashSet::new();

        for (_, val) in mapping.iter() {
            if val.len() == 1 {
                to_remove = to_remove.union(val).cloned().collect();
            }
        }

        for (_, val) in mapping.iter_mut() {
            if val.len() > 1 {
                val.retain(|x| !to_remove.contains(*x));
            }
        }
    }

    let mut final_allergens: Vec<(&str, &str)> = Vec::new();
    for (key, mut val) in mapping.drain() {
        final_allergens.push((key, val.drain().next().unwrap()));
    }
    final_allergens.sort_by(|(k1, v1), (k2, v2)| k1.cmp(k2));
    let foods: Vec<&str> = final_allergens.iter().map(|(_, v)| *v).collect();
    let mut result = String::new();

    for food in foods.iter() {
        result.push_str(*food);
        result.push(',');
    }
    result.pop().unwrap();
    println!("{:?}", result);

    Ok(())
}
