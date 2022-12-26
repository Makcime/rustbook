
#[derive(Debug)] // so we can inspect the state in a minute

enum UsState {
    Alabama,
    Alaska,
    // -snip-

}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}!", state);
            25
        },

    }
}

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32>{
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None) ;

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll(),
        _ => () // we don’t want to run any code in this case.

    }

    fn add_fancy_hat() {}
    fn reroll() {}
    fn remove_fancy_hat() {}
    fn move_player(num_space: u8) {}
}
