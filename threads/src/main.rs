use std::thread::{self, JoinHandle};
use std::time::Duration;

fn msg_hello() -> &'static str {
    thread::sleep(Duration::from_secs(1));
    "Hello, "
}

fn msg_myname() -> &'static str {
    thread::sleep(Duration::from_secs(2));
    "My name is"
}

fn msg_name() -> &'static str {
    thread::sleep(Duration::from_secs(3));
    "Tien"
}

fn msg_excited() -> &'static str {
    thread::sleep(Duration::from_secs(1));
    "!"
}

fn main() {
    let msg1 = thread::spawn(move || msg_hello());
    let msg2 = thread::spawn(move || msg_myname());
    let msg3 = thread::spawn(move || msg_name());
    let msg4 = thread::spawn(move || msg_excited());

    let msg1 = msg1.join().expect("msg1 error");
    let msg2 = msg2.join().expect("msg2 error");
    let msg3 = msg3.join().expect("msg3 error");
    let msg4 = msg4.join().expect("msg4 error");
    println!("{}", msg1);
    println!("{}", msg2);
    println!("{}", msg3);
    println!("{}", msg4);
    println!("{}{}{}{}", msg1, msg2, msg3, msg4);

}
