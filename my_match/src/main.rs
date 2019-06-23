#[derive(Debug)]

enum Indiastate {
    UP,
    MP,
    MH,
    BR,
    HP,
    PB,
}

enum Coin {
    PENNY,
    DIME,
    QUATER(Indiastate),
}

impl Coin {
    fn state_coin(&self)->u8 {
        match self {
            Coin::PENNY => 1,
            Coin::DIME => 2,
            Coin::QUATER(state) => {
                println!("The state is {:?}", state);
                3
            },
        }
    }
}

fn main() {
    let coin_value = Coin::QUATER(Indiastate::HP).state_coin();
    println!("The coin value is {}", coin_value);
}
