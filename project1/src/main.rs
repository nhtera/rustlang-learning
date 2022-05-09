

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    age: u8,
}

pub struct Students {
    class: HashMap<String, Student>,
}

impl Students {
    pub fn new() -> Self {
        Students {
            class: HashMap::new(),
        }
    }

    pub fn add(&mut self, student: Student) {
        self.class.insert(student.name.to_string(), student);
    }

    pub fn view_all(&self) -> Vec<&Student> {
        self.class.values().collect()
    }

    pub fn remove(&mut self, name: &str) -> bool {
        self.class.remove(name).is_some()
    }

    pub fn edit(&mut self, name: &str, age: u8) -> bool {
        match self.class.get_mut(name) {
            Some(name) => {
                name.age = age;
                true
            },
            None => false,
        }
    }
}

mod manager {
    use crate::{get_input, get_int, Student, Students};

    pub fn add_student(students: &mut Students) {
        println!("Enter student name: ");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Enter student age: ");
        let age = match get_int() {
            Some(input) => input,
            None => return,
        };
        let student = Student {
            name,
            age,
        };
        students.add(student);
    }

    pub fn view(students: &Students) {
        for student in students.view_all() {
            println!("{} is {} years old", student.name, student.age);
            // println!("{:?}", student);
        }
    }

    pub fn remove_student(students: &mut Students) {
        for student in Students::new().view_all() {
            println!("{}", student.name);
        }
        println!("Enter student name to remove: ");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        if students.remove(&name) {
            println!("Student {} removed", name);
        } else {
            println!("Student {} not found", name);
        }
    }

    pub fn edit_student(students: &mut Students) {
        for student in Students::new().view_all() {
            println!("{}", student.name);
        }

        println!("Enter student name: ");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        println!("Enter student age: ");
        let age = match get_int() {
            Some(input) => input,
            None => return,
        };

        if students.edit(&name, age) {
            println!("Student {} edited", name);
        } else {
            println!("Student {} not found", name);
        }
    }
}


fn get_input() -> Option<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().map(|_| input.trim().to_string())
}

fn get_int() -> Option<u8> {
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).ok().map(|_| input.trim().parse::<u8>().unwrap())

    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };

    let parsed_input = input.parse();
    match parsed_input {
        Ok(input) => Some(input),
        Err(_) => None,
    }
}

enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

impl Manager  {
    fn show() {
        println!("");
        println!("== Mangager Menu ==");
        println!("");
        println!("1. Add Student");
        println!("2. View Student");
        println!("3. Edit Student");
        println!("4. Delete Student");
        println!("");
        println!("Please enter your choice: ");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}

fn run_program() {
let mut students = Students::new();
   loop {
        Manager::show();
        let input = get_input().expect("Failed to get input");
        match  Manager::choice(&input.as_str()) {
            Some(Manager::AddStudent) => manager::add_student(&mut students),
            Some(Manager::ViewStudent) => manager::view(&students),
            Some(Manager::EditStudent) => manager::edit_student(&mut students),
            Some(Manager::DeleteStudent) => manager::remove_student(&mut students),
            None => return,
        }
   }
}

fn main() {
    run_program();
}
