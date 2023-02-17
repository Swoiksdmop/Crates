// This crate is a library
#![crate_type = "lib"]
// The library is named "rary"
#![crate_name = "math"]

pub fn public_function() {
    let x = 100 * 10 * 300;
    println!("{x}");

    if x == 300000 {
        println!("great");
    }else {
        panic!("Failed to read line");
    }
}

pub fn what() {
    println!("Good job");

    public_function();
}