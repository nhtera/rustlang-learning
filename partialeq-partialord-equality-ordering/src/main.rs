use std::cmp::{PartialOrd, Ordering};


#[derive(PartialEq)]
struct User {
    id: i32,
    name: String,
}

impl PartialOrd for User {
    // fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    //     if self.name < other.name {
    //         Some(Ordering::Less)
    //     } else if self.name > other.name {
    //         Some(Ordering::Greater)
    //     } else {
    //         Some(Ordering::Equal)
    //     }
    // }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

fn main() {
    let a = User {
        id: 1,
        name: String::from("Alice"),
    };

    let b = User {
        id: 2,
        name: String::from("Bob"),
    };

    let c = a.partial_cmp(&b);
    println!("{:?}", c);
}
