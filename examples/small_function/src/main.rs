fn main() {
    let numbers = vec![34, 50, 25, 100, 65];


    println!("The largest number is {}", find_large_number(&numbers));
}

/*
* The function has a parameter, list, which represents any concrete slice of
* i32 values that we might pass into the function.
*/
fn find_large_number(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }

    largest
}
