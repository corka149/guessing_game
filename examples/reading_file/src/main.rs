use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt"); // .unwrap or .expect("Error message")

    let mut f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => { // match guard; 
            // The ref in the pattern is needed so that error is not moved into the guard condition but is merely referenced by it. 
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => println!("File content {:?}", s),
        Err(e) => panic!("Error while reading: {:?}", e),
    }
}
