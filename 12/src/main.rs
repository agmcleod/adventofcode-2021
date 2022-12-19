use std::collections::HashMap;
use std::io::Result;

use read_input::read_text;

fn add_connection(connections: &mut HashMap<String, Vec<String>>, key: &str, value: &str) {
    if !connections.contains_key(key) {
        connections.insert(key.to_string(), Vec::new());
    }
    connections.get_mut(key).unwrap().push(value.to_string());
}

fn traverse_paths(
    path: Vec<String>,
    connections: &HashMap<String, Vec<String>>,
    can_visit_one_cave_twice: bool,
    has_visited_a_cave_twice: bool,
) -> Vec<Vec<String>> {
    let next_connections = connections.get(path.last().unwrap()).unwrap();
    let mut completed_paths = Vec::new();
    for conn in next_connections {
        if *conn == "end" {
            let mut path = path.clone();
            path.push(conn.to_owned());
            completed_paths.push(path);
        // is a small cave
        } else if *conn == conn.to_lowercase() {
            let has_visited_this_cave = path.contains(conn);
            if *conn != "start"
                && (!has_visited_this_cave
                    || (can_visit_one_cave_twice && !has_visited_a_cave_twice))
            {
                let mut has_visited_a_cave_twice = has_visited_a_cave_twice;
                if can_visit_one_cave_twice && has_visited_this_cave {
                    has_visited_a_cave_twice = true;
                }
                let mut path = path.clone();
                path.push(conn.clone());
                let mut result = traverse_paths(
                    path,
                    connections,
                    can_visit_one_cave_twice,
                    has_visited_a_cave_twice,
                );
                completed_paths.append(&mut result);
            }
        } else {
            let mut path = path.clone();
            path.push(conn.clone());
            let mut result = traverse_paths(
                path,
                connections,
                can_visit_one_cave_twice,
                has_visited_a_cave_twice,
            );
            completed_paths.append(&mut result);
        }
    }

    completed_paths
}

fn main() -> Result<()> {
    let text = read_text("12/input.txt")?;

    let mut connections = HashMap::new();

    for line in text.lines() {
        let pair = line.split("-").collect::<Vec<&str>>();
        if pair.len() != 2 {
            panic!("Could not parse line {} correctly", line);
        }
        add_connection(&mut connections, pair[0], pair[1]);
        add_connection(&mut connections, pair[1], pair[0]);
    }

    let paths = vec!["start".to_string()];

    let resulting_paths = traverse_paths(paths, &connections, false, false);
    println!("{}", resulting_paths.len());

    let paths = vec!["start".to_string()];
    let resulting_paths = traverse_paths(paths, &connections, true, false);
    println!("{}", resulting_paths.len());

    Ok(())
}
