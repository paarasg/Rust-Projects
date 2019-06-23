#[derive(Debug)]

enum Indiastate {
    HP,
    UP,
    PB,
    CH,
}

enum Coin {
    PENNY,
    DIAM,
    QUATER(Indiastate),
}

fn state_coin(coin: Coin)-> u8 {
    let mut count = 0;

    if let Coin::QUATER(Indiastate::HP) = coin {
        println!("The staue is HP");
    } else {
        count += 1;
    }
    count
}

fn main() {
    let count = state_coin(Coin::QUATER(Indiastate::UP));
    println!("{}", count);
}
