enum UsState {
    Alabama,
    Alaska,
    Mass,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(UsState::Alabama) => 77,
            Coin::Quarter(_) => 25,
        }
    }
}

fn main() {
    println!("{}", Coin::Quarter(UsState::Alabama).value_in_cents());
}
