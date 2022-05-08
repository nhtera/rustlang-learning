
#[derive(Debug)]
struct Member {
    username: String,
    email: String,
    age: u64,
    active: bool,
}


fn main() {
    let mut member = Member {
        username: String::from("John"),
        email: String::from("xxx@gmail.com"),
        age: 30,
        active: true,
    };

    let name = member.username;
    println!("{}", name);
    member.username = String::from("Tien");
    println!("{}", member.username);

    let member2 = create_new_member(String::from("Tung"), String::from("sksskk@gmail.com"), 30);
    println!("{:#?}", member2);

    let member3 = Member {
        username: String::from("Hai"),
        ..member2
    };
    println!("{:#?}", member3);

}

fn create_new_member(username: String, email:String, age: u64) -> Member {
    Member {
        username,
        email,
        age,
        active: true,
    }
}
