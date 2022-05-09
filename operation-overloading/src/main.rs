use std::ops::{Add, Index};


#[derive(Debug)]
struct Letter(char);

impl Add<Self> for Letter {
    type Output = String;
    fn add(self, input: Self) -> Self::Output {
        format!("{}{}", self.0, input.0)
    }
}


enum Vinfast {
    Vf6,
    Vf7,
    Vf8,
    Vf9,
}

struct Price {
    price_vf6: i64,
    price_vf7: i64,
    price_vf8: i64,
    price_vf9: i64,
}

impl Index<Vinfast> for Price {
 type Output = i64;
    fn index(&self, index: Vinfast) -> &Self::Output {
        match index {
            Vinfast::Vf6 => &self.price_vf6,
            Vinfast::Vf7 => &self.price_vf7,
            Vinfast::Vf8 => &self.price_vf8,
            Vinfast::Vf9 => &self.price_vf9,
        }
    }
}

fn main() {
    let a = Letter('a');
    let b = Letter('b');
    println!("{}", a + b);

    let price = Price {
        price_vf6: 100,
        price_vf7: 200,
        price_vf8: 300,
        price_vf9: 400,
    };

    let find = price[Vinfast::Vf7];
    println!("{}", find);
}
