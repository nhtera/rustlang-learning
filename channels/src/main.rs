 use crossbeam_channel::unbounded;
 use std::thread;

#[derive(Debug)]
enum Message {
    PrintMsg(String),
    Sum(i32, i32),
    Quit,
}

enum MainMsg {
    ResultSum(i32),
    MainQuit,
}

fn main() {
   let (sender, receiver) = unbounded();
   let (sender2, receiver2) = unbounded();

//    let a = thread::spawn (move || match receiver.recv() {
//     Ok(message) => println!("{}", message),
//     Err(e) => println!("Error: {}", e),
//    });

//    sender.send("Hello, world!");
//    a.join();

    let a = thread::spawn (move ||  loop {
        match receiver.recv() {
            Ok(message) => match message {
                Message::PrintMsg(s) => println!("{}", s),
                Message::Sum(a, b) => {
                    // println!("{}", a + b)
                    sender2.send(MainMsg::ResultSum(a + b));
                },
                Message::Quit => {
                    println!("Quitting...");
                    sender2.send(MainMsg::MainQuit);
                    break;
                },
            },
            Err(e) => {
                println!("Error: {}", e);
                sender2.try_send(MainMsg::MainQuit);
                break;
            },
        }
    });
    sender.send(Message::PrintMsg("Hello, world!".to_string()));
    sender.send(Message::Sum(1, 2));
    sender.send(Message::Quit);

    while let Ok(msg) = receiver2.recv() {
        match msg {
            MainMsg::ResultSum(a) => println!("{}", a),
            MainMsg::MainQuit => {
                println!("Main quitting...");
                break;
            },
        }
    }
    a.join();
}
