use std::collections::HashMap;
use std::io::Result;

use read_input::read_text;

fn insert_or_append_count(
    map: &mut HashMap<(char, char), usize>,
    key: &(char, char),
    count: usize,
) {
    if map.contains_key(&key) {
        *map.get_mut(&key).unwrap() += count;
    } else {
        map.insert(key.to_owned(), count);
    }
}

fn solve_for_n_steps(
    rules: &HashMap<(char, char), char>,
    mut template: HashMap<(char, char), usize>,
    steps: usize,
    mut ending_pair: (char, char),
) {
    for _ in 0..steps {
        let mut next_template = template.clone();
        for (pair, count) in &template {
            if rules.contains_key(pair) {
                let next_count = next_template.get_mut(pair).unwrap();
                *next_count -= *count;
                if *next_count == 0 {
                    next_template.remove(pair);
                }
                let transform = rules.get(pair).unwrap();
                insert_or_append_count(&mut next_template, &(pair.0, *transform), *count);
                insert_or_append_count(&mut next_template, &(*transform, pair.1), *count);
                if ending_pair == *pair {
                    ending_pair = (*transform, pair.1);
                }
            }
        }
        template = next_template;
    }

    let mut count_per_char = HashMap::new();
    for ((l1, l2), count) in &template {
        if count_per_char.contains_key(l1) {
            *count_per_char.get_mut(l1).unwrap() += *count;
        } else {
            count_per_char.insert(*l1, *count);
        }

        if ending_pair.0 == *l1 && ending_pair.1 == *l2 {
            if count_per_char.contains_key(l2) {
                *count_per_char.get_mut(l2).unwrap() += 1;
            } else {
                count_per_char.insert(*l2, 1);
            }
        }
    }

    let mut min = std::usize::MAX;
    let mut max = std::usize::MIN;

    for (_el, count) in &count_per_char {
        min = min.min(*count);
        max = max.max(*count);
    }

    println!("{}", max - min);
}

fn main() -> Result<()> {
    let text = read_text("14/input.txt")?;

    let mut rules = HashMap::new();

    let mut template = HashMap::new();
    let mut retrieved_template = false;
    let mut ending_pair = ('a', 'b');
    for line in text.lines() {
        if retrieved_template {
            if line != "" {
                let mut iter = line.split(" -> ");
                let pattern = iter.next().unwrap().to_owned();
                if pattern.len() != 2 {
                    panic!("Pattern is incorrect length {}", line);
                }
                let pattern: Vec<char> = pattern.chars().collect();
                let to_insert = iter.next().unwrap();
                if to_insert.len() != 1 {
                    panic!("Transformed value is incorrect length {}", line);
                }
                let to_insert = to_insert.chars().next().unwrap();

                rules.insert((pattern[0], pattern[1]), to_insert);
            }
        } else {
            let letters = line.chars().collect::<Vec<char>>();
            for pair in letters.windows(2) {
                let key = (pair[0], pair[1]);
                ending_pair = key.clone();
                insert_or_append_count(&mut template, &key, 1);
            }
            retrieved_template = true;
        }
    }

    solve_for_n_steps(&rules, template.clone(), 10, ending_pair.clone());
    solve_for_n_steps(&rules, template.clone(), 40, ending_pair);

    Ok(())
}
