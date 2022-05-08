struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![34, 50, 25, 100, 65];
    let largest1 = get_largest(number_list);
    println!("The largest number is {}", largest1);

    println!("-------------------------------");
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = Point { x: 5, y: 10.43 };

    let p4 = p1.mixup(p2);
    println!("p4.x = {}, p4.y = {}", p4.x, p4.y);

}

fn get_largest<T: PartialOrd + Copy> (number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
