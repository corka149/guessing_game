#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny(u32),
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny(number) => {
            println!("Serial number {}", number);
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        },
    }
}

fn plus_one(x : Option<i32>) -> Option<i32> {
    match x {
        Some(t) => Some(t + 1),
        None => None,
    }
}

fn main() {
    // let penny = Coin::Penny;
    let val = value_in_cents(Coin::Quarter(UsState::Alabama));

    println!("The the value is {}", val);

    let serial_number: u32 = 1234567;
    value_in_cents(Coin::Penny(serial_number));

    // ------------
    let opt1 = Some(5);
    let opt2 : Option<i32> = None;

    plus_one(opt1);
    plus_one(opt2);

    // -----------
    let my_value : u8 = 3;
    match my_value {
        1 => println!("First!"),
        _ => (), // _ is default/placeholder
    }

    //-----------
    if let Some(5) = opt1 {
        println!("It is five!");
    }

    if let Some(value) = opt1 {
        println!("Value is {}", value);
    }
}
