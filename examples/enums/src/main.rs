#![allow(unused_variables)]
enum IpAddrKind{            
    //V4,
    //V6,

    // This definition of the IpAddr enum says that both V4 and V6 variants will have associated String values
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let my_message = Message::Write(String::from("Greetings to the earth"));
    my_message.call();

    // The enum Option::Some and Option::None
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}
