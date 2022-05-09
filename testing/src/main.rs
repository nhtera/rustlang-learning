fn main() {
    println!("Hello, world!");
}

fn caps(input: &str) -> String {
    input.to_uppercase()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check() {
        let result = caps("Hello, world!");
        let expected = "HELLO, WORLD!";
        assert_eq!(result, expected, "String should be all upper case");
    }
}
