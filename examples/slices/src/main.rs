fn main() {
   // let byte1 = b'a';
   // let byte2 = b' ';
   // println!("'{}','{}'",byte1, byte2);

   let my_string = String::from("hello world");

   // first word works on slices of 'String's
   let word = first_word(&my_string[..]);

   let my_string_literal = "hello world";

   // first_word works on string literals
   let word = first_word(&my_string_literal[..]);

   // since string literals *are* string slices already,
   // this works too, without the slice syntax!
   let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { // iter returns each element in a collection
        if item == b' ' {                         // b' ' = byte literal syntax
            return &s[0..i];           
        }
    }

    &s[..]
}