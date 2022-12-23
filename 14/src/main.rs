use std::collections::{HashMap, LinkedList};
use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("14/input.txt")?;

    let mut rules = HashMap::new();

    let mut template = LinkedList::new();
    let mut retrieved_template = false;
    for line in text.lines() {
        if retrieved_template {
            if line != "" {
                let mut iter = line.split(" -> ");
                let pattern = iter.next().unwrap().to_owned();
                if pattern.len() != 2 {
                    panic!("Pattern is incorrect length {}", line);
                }
                let to_insert = iter.next().unwrap().to_owned();

                rules.insert(pattern, to_insert);
            }
        } else {
            for letter in line.split("") {
                if letter != "" {
                    template.push_back(letter.to_string());
                }
            }
            retrieved_template = true;
        }
    }

    // or 10 for part 1
    for _ in 0..10 {
        let mut next_iteration = template.clone();
        let mut insert_count = 0;
        for i in 0..template.len() {
            let iter = template.iter();
            let mut iter = iter.skip(i);
            let first = iter.next().unwrap();
            let second = iter.next();
            // second can be empty if we're at the end of the list
            if second.is_none() {
                break;
            }
            let second = second.unwrap();
            if let Some(insert_value) = rules.get(&format!("{}{}", first, second)) {
                let mut second_part = next_iteration.split_off(i + insert_count + 1);
                insert_count += 1;
                second_part.push_front(insert_value.to_owned());
                next_iteration.append(&mut second_part);
            } else {
                panic!("No rule found for pair {}{}", first, second);
            }
        }

        template = next_iteration;
    }

    let mut count = HashMap::new();
    for el in &template {
        if count.contains_key(el) {
            *count.get_mut(el).unwrap() += 1;
        } else {
            count.insert(el.to_owned(), 1);
        }
    }

    let mut min = std::i32::MAX;
    let mut max = std::i32::MIN;

    for (_el, count) in &count {
        min = min.min(*count);
        max = max.max(*count);
    }

    println!("{}", max - min);

    Ok(())
}
