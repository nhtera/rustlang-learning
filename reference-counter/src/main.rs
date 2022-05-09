
use std::rc::Rc;


struct Car {
    number: String,
}

struct Door {
    vehicle: Rc<Car>,
}

fn main() {
    let car = Rc::new(Car { number: "123".to_string() });
    let left_door = Door {
        vehicle: Rc::clone(&car),
    };

    let right_door = Door {
        vehicle: Rc::clone(&car),
    };

    drop(car);
    println!("{}", left_door.vehicle.number);
    drop(left_door);
    println!("{}", right_door.vehicle.number);
}
