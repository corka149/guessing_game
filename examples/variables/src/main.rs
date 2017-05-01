const Y: u32 = 12;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    // shadowing
    let x = 6;
    println!("The value of x is: {}", x);

    println!("The value of const y is: {}:", y);

    //---------------------------------------------------- Compound Types
    // tuple: Elements can have different types.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // getting out the values with a pattern
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // arrays: All elements must have the same type.
    let a = [1,2,3,4,5];
    let first = a[0];

    
}
