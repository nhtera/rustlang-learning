use parking_lot::Mutex;
use std::{sync::Arc, thread};

struct Number (usize);

fn main() {
    let number = Number(0);
    let number_thread = Arc::new(Mutex::new(number));

    let number_thead1 = Arc::clone(&number_thread);
    let number_thead2 = number_thread.clone();

    let thread1 = thread::spawn(move || {
        let mut number_plus = number_thead1.lock();
        number_plus.0 += 20;
    });

    let thread2 = thread::spawn(move || {
        let mut number_plus = number_thead2.lock();
        number_plus.0 += 32;
    });

    thread1.join().and_then(|_| thread2.join());
    println!("{}", number_thread.lock().0);
}
