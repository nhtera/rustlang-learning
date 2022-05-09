fn main() {
    println!("Hello, world!");
    let num1 = 10;
    let num2 = 20;
    let result = get_ref(&num1, &num2);
    println!("{}", result);
}

fn get_ref<'a>(param1: &'a i32, param2: &'a i32) -> &'a i32 {
    if param1 > param2 {
        param1
    } else {
        param2
    }
}

fn test1<'a, 'b: 'a>(param1: i32, param2: &'a str, param3: &'b str, param4: i32) -> &'a str {
    if param1 ==7 && param4 > 10 {
        param2
    } else {
        param3
    }
}


// #[allow(dead_code)]
// fn test_1(param1: Vec<f64>) -> Vec<f64> {
//     param1
// }

// #[allow(dead_code)]
// fn test_2<'a>(param1: &Vec<f64>) -> Vec<f64> {
//     param1
// }

// #[allow(dead_code)]
// fn test_3(param1: &Vec<f64>) -> &Vec<f64> {
//     &param1
// }