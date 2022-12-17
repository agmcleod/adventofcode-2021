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
    mut path: Vec<String>,
    connections: &HashMap<String, Vec<String>>,
    cave: &String,
) -> Vec<Vec<String>> {
    let next_connections = connections.get(cave).unwrap();
    let mut completed_paths = Vec::new();
    for conn in next_connections {
        if *conn == "end" {
            let mut path = path.clone();
            path.push(conn.to_owned());
            completed_paths.push(path);
        // is a small cave
        } else if *conn == conn.to_lowercase() {
        } else {
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

    let mut next_connections = connections.get("start").unwrap();
    let mut paths = Vec::new();
    paths.push(vec!["start".to_string()]);

    traverse_paths(paths, &connections, "start");

    Ok(())
}
