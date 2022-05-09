struct Employee {
    position: Position,
    status: Status,
}

enum Position {
    IT,
    CEO,
    CTO,
    Manager,
    Maketer,
}

enum Status {
    Active,
    Denied,
}

fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Denied => return Err("Access denied".to_string()),
        _ => (),
    }

    match employee.position {
        Position::CEO => Ok(()),
        Position::CTO => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Invalid position".to_string()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    let _ = try_access(employee)?;
    println!("access");
    Ok(())
}

fn main() {

    let manager = Employee {
        position: Position::Manager,
        status: Status::Active,
    };

    let it = Employee {
        position: Position::IT,
        status: Status::Active,
    };

    // let emp = try_access(&manager);
    // println!("{:?}", emp);
    print_access(&manager);
    println!("-------------------------------");
    print_access(&it);
}
