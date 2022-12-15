use std::collections::HashMap;
use std::io::Result;

use read_input::read_text;

fn add_connection(connections: &mut HashMap<String, Vec<String>>, key: &str, value: &str) {
    if !connections.contains_key(key) {
        connections.insert(key.to_string(), Vec::new());
    }
    connections.get_mut(key).unwrap().push(value.to_string());
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
    loop {
        for conn in next_connections {}

        if next_connections.len() == 0 {
            break;
        }
    }

    Ok(())
}
