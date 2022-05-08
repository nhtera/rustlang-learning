fn main() {
    // Data type
    // Scalar Data

    println!("----------Variable-------------");
    let mut x = 10;
    println!("x = {}", x);
    x = 20;
    println!("x = {}", x);
    println!("-----------------------");

    // Constant
    println!("----------Constant-------------");
    const HANG_SO: u128 = 100_000_000_000_000;
    println!("Hang so = {}", HANG_SO);
    println!("-----------------------");

    // Shadowing
    println!("----------Shadowing-------------");
    let x1 = 10;
    println!("x1 = {}", x1);
    let x1 = "Ten";
    println!("x1 = {}", x1);
    println!("----------");

    let outer = 10; {
        let innter = 200;
        println!("inner = {}", innter);
        let outer = 300;
        println!("outer = {}", outer);
    }
    println!("outer = {}", outer);

    println!("-------------Datatype-------------");

    println!("-------------Interger-------------");
    let x2: u8 = 255;
    println!("x2 = {}", x2);
    let a = 111_111; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_1111; // Binary
    let e = b'A'; // Byte (u8 only)
    println!("{}, {}, {}, {}, {}", a, b, c, d, e);
    println!("-------------Float-------------");
    let f = 2.0;
    let g = 3.0;
     let sum = 3 + 4;
     let subtration = 5 - 4;
     let multiple = 3 * 5;
     let division = 40.4 / 20.4;
     let remainder = 43 % 4;
    println!("{}, {}, {}, {}, {}, {}, {}", f, g, sum, subtration, multiple, division, remainder);

    println!("-------------Boolean-------------");
    let b1 = true;
    let b2: bool = false;
    println!("{}, {}", b1, b2);

    println!("-------------Character-------------");
    let c1 = 'A';
    let icon = '\u{1F600}';
    let icon2 = '😀';
    println!("{}, {}, {}", c1, icon, icon2);

    // Compound Data
    // Tuple
    println!("-------------Tuple-------------");
    let t1 = ("Hello", 100_000, 3, 4, 5);
    println!("{:?}", t1);
    let (t2, t3, t4, t5, t6) = t1;
    let i1 = t1.1;
    println!("{}", i1);
    println!("{}, {}, {}, {}, {}", t2, t3, t4, t5, t6);

    // Array
    println!("-------------Array-------------");
    let a1 = [1, 2, 3, 4, 5];
    println!("{:?}", a1);
    let hasing = [0; 32];
    println!("{:?}", hasing);
    for i in hasing.iter() {
        print!("{}", i);
    }
    print!("\n");

}
