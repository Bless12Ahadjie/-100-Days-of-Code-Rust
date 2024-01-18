
use std::io;

fn main() {
    let mut age = String::new();

    println!("How old are you mate? ");

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    let age: u32 = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error");
            0 // or any default value you want to assign in case of an error
        }
    };

    println!("You are {} years old", age);
}

