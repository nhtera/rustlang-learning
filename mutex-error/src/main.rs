use std::{rc::Rc, sync::Arc, thread, time::Duration};
use parking_lot::Mutex;
use backoff::ExponentialBackoff;

type ArcAccount = Arc<Mutex<Account>>;

struct Account {
    balance: usize,
}

fn transfer(from: ArcAccount, to: ArcAccount, amount: usize) {
    let op = || {
        if let Some(mut from) = from.try_lock() {
            if let Some(mut to) = to.try_lock() {
                from.balance -= amount;
                to.balance += amount;
                return Ok(());
            }
        }
        Err(0)?
    };

    let backoff = ExponentialBackoff::default();
    backoff::retry(backoff, op);
}

fn main() {

    // let a = ArcAccount(Arc::new(Mutex::new(Account { balance: 100 })));
    // let b = ArcAccount(Arc::new(Mutex::new(Account { balance: 200 })));
    // let transaction_1 = thread.spawn(move || {
    //     transfer(a, b, 100);
    // });

    // let transaction_2 = thread.spawn(move || {
    //     transfer(b, a, 100);
    // });
}
