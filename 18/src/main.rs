use std::io::Result;
use std::str::Chars;

use read_input::read_text;

#[derive(PartialEq)]
enum Pair {
    None,
    Pair(Box<(Pair, Pair)>),
    Value(u32),
}

fn create_pair_structure(iter: &mut Chars) -> Pair {
    let pair: Pair = Pair::Pair(Box::new((Pair::None, Pair::None)));

    loop {
        let ch = iter.next();
        if ch.is_none() {
            break;
        }

        let ch = ch.unwrap();
        match ch {
            '[' => match create_pair_structure(iter) {
                Pair::None => {
                    panic!("Returned None after a left bracket.");
                }
                Pair::Pair(returned_pair) => match pair {
                    Pair::Pair(mut pair) => {
                        if pair.0 == Pair::None {
                            pair.0 = Pair::Pair(returned_pair);
                        } else if pair.1 == Pair::None {
                            pair.1 = Pair::Pair(returned_pair);
                        } else {
                            panic!(
                                "Pair already populated for trying to populate returned pair from sub level"
                            );
                        }
                    }
                    _ => panic!("unexpected non-pair type for this level's pair value"),
                },
                Pair::Value(_value) => {
                    panic!("Should not have returned single value");
                }
            },
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
                match pair {
                    Pair::Pair(mut pair) => {
                        if pair.0 == Pair::None {
                            pair.0 = Pair::Value(digit.unwrap());
                        } else if pair.1 == Pair::None {
                            pair.1 = Pair::Value(digit.unwrap());
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

fn main() -> Result<()> {
    let text = read_text("18/input.txt")?;

    for line in text.lines() {}

    Ok(())
}
