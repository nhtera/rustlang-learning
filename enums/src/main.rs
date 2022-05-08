fn main() {
//    let x = 5;
//    let y = Some(5);
//    let sum = x + y.unwrap_or(0);
//    println!("{}", sum);

    decimals(Coin::Bitcoin(Balance::Medium));

}

#[derive(Debug)]
enum Balance {
    Small,
    Medium,
    Fish,
    Shark,
}

enum Coin {
    Solana,
    Ethereum,
    Near,
    Bitcoin(Balance),
}

fn decimals(coin: Coin) -> u8 {
    match coin {
        Coin::Solana => {
            println!("Solana");
            18
        },
        Coin::Ethereum => 18,
        Coin::Near => 12,
        Coin::Bitcoin(bala) => {
            println!("Bitcoin: {:#?}", bala);
            8
        },
    }
}