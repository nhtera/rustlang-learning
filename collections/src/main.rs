use std::collections::HashMap;

fn main() {
    // Vector
    let a = [1, 2, 3];
    let v = vec![1, 2, 3];
    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(4);
    v2.push(5);
    println!("{:?}", v2);

    let four = &v2[4];

    match v2.get(3) {
        Some(four) => println!("{}", four),
        None => println!("None"),
    }

    for i in &mut v2 {
        *i += 10
    }

    for i in &v2 {
        println!("{}", i);
    }

    println!("-------------------------------");

    enum SheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SheetCell::Int(3),
        SheetCell::Float(10.12),
        SheetCell::Text(String::from("Hello")),
    ];

    match &row[1] {
        &SheetCell::Float(f) => println!("{}", f),
        _ => println!("This is not a float"),
    }

    println!("-------------------------------");

    // String
    let s1 = String::from("Hello, world!");
    let s2 = String::new();
    let s3 = "A string";
    let s4 = s3.to_string();

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s4);

    let s5 = s1 + " hihihi " + &s3;
    println!("{}", s5);
    println!("{}", s3);

    // Bytes
    for i in s3.bytes() {
        println!("{}", i);
    }

    // Scalar values
    for i in s3.chars() {
        println!("{}", i);
    }

    println!("-------------------------------");
    // HashMap
    let mu = String::from("MU");
    let mc = String::from("MC");
    let mut scores = HashMap::new();
    scores.insert(mu, 10);
    scores.insert(mc, 9);

    let team_name = String::from("MU");
    let score = scores.get(&team_name);
    println!("Score: {:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("MU"), 10);
    scores2.insert(String::from("MU"), 20);

    scores2.entry(String::from("MC")).or_insert(10);
    scores2.entry(String::from("MC")).or_insert(20);
    println!("{:?}", scores2);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for i in text.split_whitespace() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}
