fn main() {
    let data: Vec<_> = vec![1, 2, 3, 4, 5]
    .iter() // create a iterator
    .map(|num| num * 3) // itarate all elements and multi 3
    .filter(|num| num > &10) // filter all elements  > 10
    .collect(); // collect all elements to a vector

    for num in data {
        println!("{}", num);
    }



    // Closures function
    let sum = |a: i32, b: i32| a + b;
    println!("{}", sum(1, 2));

}
