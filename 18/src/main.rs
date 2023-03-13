use std::cell::RefCell;
use std::fmt;
use std::io::Result;
use std::rc::{Rc, Weak};
use std::str::Chars;

use read_input::read_text;
use uuid::Uuid;

enum Pair {
    None,
    PairNode(PairNode),
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
            Pair::PairNode(pair) => {
                write!(f, "{}", pair)
            }
            Pair::Value(n) => write!(f, "{}", n),
        }
    }
}

struct PairNode {
    id: String,
    parent: Option<Weak<RefCell<PairNode>>>,
    children: (Rc<RefCell<Pair>>, Rc<RefCell<Pair>>),
}

impl PairNode {
    fn new(parent: Option<Weak<RefCell<PairNode>>>) -> Self {
        PairNode {
            id: Uuid::new_v4().to_string(),
            parent,
            children: (
                Rc::new(RefCell::new(Pair::None)),
                Rc::new(RefCell::new(Pair::None)),
            ),
        }
    }
}

impl PartialEq for PairNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for PairNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{},{}]",
            self.children.0.as_ref().borrow(),
            self.children.1.as_ref().borrow()
        )
    }
}

fn create_pair_structure(iter: &mut Chars, pair_node: PairNode) -> Rc<RefCell<PairNode>> {
    let pair = Rc::downgrade(&Rc::new(RefCell::new(pair_node)));

    loop {
        let ch = iter.next();
        if ch.is_none() {
            break;
        }

        let ch = ch.unwrap();
        match ch {
            '[' => {
                let returned_pair = create_pair_structure(
                    iter,
                    // create a new pair to pass down to be updated by the recursive call
                    PairNode::new(Some(pair.clone())),
                );
                let pair = pair.upgrade().unwrap();
                let mut pair = pair.borrow_mut();
                if pair.children.0.as_ref().borrow().is_none() {
                    pair.children.0 =
                        Rc::new(RefCell::new(Pair::PairNode(returned_pair.into_inner())));
                } else if pair.children.1.as_ref().borrow().is_none() {
                    pair.children.1 =
                        Rc::new(RefCell::new(Pair::PairNode(returned_pair.into_inner())));
                } else {
                    panic!(
                        "Pair already populated for trying to populate returned pair from sub level"
                    );
                }
            }
            ']' => break,
            ',' => {
                // no op, we just continue with the pair
            }
            _ => {
                let digit = ch.to_digit(10);
                if digit.is_none() {
                    panic!("Invalid number: {}", ch);
                }

                let pair = pair.upgrade().unwrap();
                let mut pair = pair.borrow_mut();
                if pair.children.0.as_ref().borrow().is_none() {
                    pair.children.0 = Rc::new(RefCell::new(Pair::Value(digit.unwrap())));
                } else if pair.children.1.as_ref().borrow().is_none() {
                    pair.children.1 = Rc::new(RefCell::new(Pair::Value(digit.unwrap())));
                } else {
                    panic!(
                        "Pair already populated for trying to populate number {}",
                        digit.unwrap()
                    );
                }
            }
        }
    }

    pair.upgrade().unwrap()
}

// fn reduce_pair(pair: &mut Pair, depth: u32) {
//     if depth > 5 {
//         panic!("Unexpected depth level: {}", depth);
//     }
//     match pair {
//         Pair::PairNode(pair) => {
//             let next_depth = depth + 1;

//             // instead of nesting let's expload the pair
//             if next_depth == 5 {
//                 // get left of pair
//                 let mut going_up = true;
//                 let mut parent = pair.parent.unwrap().as_ptr().as_ptr();
//                 loop {
//                     if going_up {}
//                 }
//             } else {
//                 reduce_pair(&mut pair.0.pair, next_depth);
//                 reduce_pair(&mut pair.1.pair, next_depth);
//             }
//         }
//         _ => {}
//     }
// }

fn main() -> Result<()> {
    let text = read_text("18/input.txt")?;

    for line in text.lines() {
        let mut iter = line.chars();
        let mut pair = create_pair_structure(&mut iter, PairNode::new(None));
        println!("{}", pair.into_inner());
        // reduce_pair(&mut pair, 1);
    }

    Ok(())
}
