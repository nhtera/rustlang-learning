// mod front_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         pub fn seat_at_table() {}
//     }

//     pub mod serving {
//         pub fn take_order() {}
//         pub fn take_payment() {}
//     }
// }

mod front_house;

fn call_order(){}
mod test {
    mod back_house {
        fn cook_order() {}
        fn fix_order() {
            super::super::call_order();
            cook_order();
        }
    }
}

mod back_house {
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
    }

    pub enum Salad {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn monday(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("orange"),
            }
        }
    }
}

fn eat_at_restaurant() {
    crate::front_house::hosting::add_to_waitlist();

    let mut order = back_house::Breakfast::monday("Rye");
    order.toast = String::from("Wheat");

    let order1 = back_house::Breakfast {
        toast: String::from("Rye"),
        fruit: String::from("orange"),
    };

    let order2 = back_house::Salad::Soup;
}