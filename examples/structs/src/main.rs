#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {                // impl = implementation
    fn area(&self) -> u32 {     // a method
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {     // a method
        self.length > other.length && self.width > other.width 
    }

    fn square(len: u32) -> Rectangle {      // An associated function
        Rectangle{ 
            length: len, 
            width: len
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    let rect2 = Rectangle {
        length: 40,
        width: 20,
    };
    let rect3 = Rectangle {
        length: 40,
        width: 40,
    };
    let sq = Rectangle::square(50);

    println!("rect1 is {:?}", rect1);
    println!("The are of the rectangle is {} square pixels.",
             rect1.area());

    println!("2 in 1? {}", rect1.can_hold(&rect2));
    println!("3 in 1? {}", rect1.can_hold(&rect3));

    println!("My square {:#?}", sq);
}
