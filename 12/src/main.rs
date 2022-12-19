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
            if !path.contains(conn) {
                let mut path = path.clone();
                path.push(conn.clone());
                let mut result = traverse_paths(path, connections);
                completed_paths.append(&mut result);
            }
        } else {
            let mut path = path.clone();
            path.push(conn.clone());
            let mut result = traverse_paths(path, connections);
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

    let resulting_paths = traverse_paths(paths, &connections);
    println!("{}", resulting_paths.len());

    Ok(())
}
