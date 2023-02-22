use std::fmt;
use std::io::Result;
use std::str::Chars;

use read_input::read_text;
use uuid::Uuid;

enum Pair {
    None,
    Pair(Box<(PairNode, PairNode)>),
    Value(u32),
}

impl Pair {
    fn is_none(&self) -> bool {
        match *self {
            Pair::None => true,
            _ => false,
        }
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pair::None => {
                write!(f, "")
            }
            Pair::Pair(pair) => {
                write!(f, "[{},{}]", pair.0.pair, pair.1.pair)
            }
            Pair::Value(n) => write!(f, "{}", n),
        }
    }
}

enum TreeSide {
    Left,
    Right,
}

struct PairNode {
    id: String,
    parent: Box<Pair>,
    side_of_parent: TreeSide,
    pair: Box<Pair>,
}

impl PairNode {
    fn new(parent: Box<Pair>, side_of_parent: TreeSide) -> Self {
        PairNode {
            id: Uuid::new_v4().to_string(),
            parent,
            side_of_parent,
            pair: Box::new(Pair::None),
        }
    }

    fn is_none(&self) -> bool {
        self.pair.is_none()
    }
}

impl PartialEq for PairNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn create_pair_structure(iter: &mut Chars, mut pair: Pair) -> Pair {
    loop {
        let ch = iter.next();
        if ch.is_none() {
            break;
        }

        let ch = ch.unwrap();
        match ch {
            '[' => {
                match create_pair_structure(
                    iter,
                    // create a new pair to pass down to be updated by the recursive call
                    Pair::Pair(Box::new((
                        PairNode::new(Box::new(pair), TreeSide::Left),
                        PairNode::new(Box::new(pair), TreeSide::Right),
                    ))),
                ) {
                    Pair::None => {
                        panic!("Returned None after a left bracket.");
                    }
                    Pair::Pair(returned_pair) => match &mut pair {
                        Pair::Pair(pair) => {
                            if pair.0.is_none() {
                                pair.0.pair = Box::new(Pair::Pair(returned_pair));
                            } else if pair.1.is_none() {
                                pair.1.pair = Box::new(Pair::Pair(returned_pair));
                            } else {
                                panic!(
                                "Pair already populated for trying to populate returned pair from sub level"
                            );
                            }
                        }
                        Pair::None => {
                            pair = Pair::Pair(returned_pair);
                        }
                        _ => panic!("unexpected value type for this level's pair value"),
                    },
                    Pair::Value(_value) => {
                        panic!("Should not have returned single value");
                    }
                }
            }
            ']' => return pair,
            ',' => {
                // no op, we just continue with the pair
            }
            _ => {
                let digit = ch.to_digit(10);
                if digit.is_none() {
                    panic!("Invalid number: {}", ch);
                }

                // with a digit parsed, we need to add it to this level's pair
                match &mut pair {
                    Pair::Pair(pair) => {
                        if pair.0.is_none() {
                            pair.0.pair = Box::new(Pair::Value(digit.unwrap()));
                        } else if pair.1.is_none() {
                            pair.1.pair = Box::new(Pair::Value(digit.unwrap()));
                        } else {
                            panic!(
                                "Pair already populated for trying to populate number {}",
                                digit.unwrap()
                            );
                        }
                    }
                    _ => panic!("unexpected non-pair type for this level's pair value"),
                }
            }
        }
    }

    pair
}

fn reduce_pair(pair: &mut Pair, depth: u32) {
    if depth > 5 {
        panic!("Unexpected depth level: {}", depth);
    }
    match pair {
        Pair::Pair(pair) => {
            let next_depth = depth + 1;

            // instead of nesting let's expload the pair
            if next_depth == 5 {
                // get left of pair
                let mut going_up = true;
                let mut parent = pair.0.parent;
                loop {
                    if going_up {}
                }
            } else {
                reduce_pair(&mut pair.0.pair, next_depth);
                reduce_pair(&mut pair.1.pair, next_depth);
            }
        }
        _ => {}
    }
}

fn main() -> Result<()> {
    let text = read_text("18/input.txt")?;

    for line in text.lines() {
        let mut iter = line.chars();
        let mut pair = create_pair_structure(&mut iter, Pair::None);
        // println!("{}", pair);
        reduce_pair(&mut pair, 1);
    }

    Ok(())
}
