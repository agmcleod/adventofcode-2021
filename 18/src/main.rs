use std::cell::RefCell;
use std::fmt;
use std::io::Result;
use std::rc::{Rc, Weak};
use std::str::Chars;

use read_input::read_text;
use uuid::Uuid;

struct PairNode {
    id: String,
    parent: Option<Weak<RefCell<PairNode>>>,
    left: Option<Rc<RefCell<PairNode>>>,
    right: Option<Rc<RefCell<PairNode>>>,
    value: Option<u32>,
}

impl PairNode {
    fn new(parent: Option<Weak<RefCell<PairNode>>>) -> Self {
        PairNode {
            id: Uuid::new_v4().to_string(),
            parent,
            left: None,
            right: None,
            value: None,
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
        if self.value.is_some() {
            write!(f, "{}", self.value.unwrap())
        } else {
            if self.left.is_some() && self.right.is_some() {
                write!(
                    f,
                    "[{},{}]",
                    self.left.as_ref().unwrap().borrow(),
                    self.right.as_ref().unwrap().borrow()
                )
            } else if self.left.is_some() {
                write!(f, "[{},]", self.left.as_ref().unwrap().borrow())
            } else {
                write!(f, "[,{}]", self.right.as_ref().unwrap().borrow())
            }
        }
    }
}

fn create_pair_structure(iter: &mut Chars, parent_pair_node: PairNode) -> Rc<RefCell<PairNode>> {
    let parent = Rc::downgrade(&Rc::new(RefCell::new(parent_pair_node)));

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
                    PairNode::new(Some(parent.clone())),
                );
                let parent = parent.upgrade().unwrap();
                let mut parent = parent.borrow_mut();
                if parent.left.is_none() {
                    parent.left = Some(returned_pair);
                } else if parent.right.is_none() {
                    parent.right = Some(returned_pair);
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

                let pair = parent.upgrade().unwrap();
                let mut pair = pair.borrow_mut();
                if pair.left.is_none() {
                    let mut child_node = PairNode::new(Some(parent));
                    child_node.value = Some(digit.unwrap());
                    pair.left = Some(Rc::new(RefCell::new(child_node)));
                } else if pair.right.is_none() {
                    let mut child_node = PairNode::new(Some(parent));
                    child_node.value = Some(digit.unwrap());
                    pair.right = Some(Rc::new(RefCell::new(child_node)));
                } else {
                    panic!(
                        "Pair already populated for trying to populate number {}",
                        digit.unwrap()
                    );
                }
            }
        }
    }

    parent.upgrade().unwrap()
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
