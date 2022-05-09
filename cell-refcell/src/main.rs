use std::cell::RefCell;

#[derive(Debug)]
struct Channel {
    name: RefCell<String>,
}

fn main() {
    let my_channel = Channel {
        name: RefCell::new("rust".to_string()),
    };

    {
        let mut a = my_channel.name.borrow_mut();
        *a = "rustlang".to_string();
    }

    {
        my_channel.name.replace("Tien".to_string());
    }

    
    println!("{:?}", my_channel.name);
}
