
use std::io;

fn main() {
    let mut age = String::new();

    loop{
        
    println!("How old are you mate? ");

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    let age: u32 = match age.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Error {}",e);
            0 
        }
    };

    if age >= 18 {
        println!("You are eligible to vote");
    }
    else if age < 18 {
        println!("You are not eligible to vote");
    }
    else {
        println!("Hey mate ! I think you are an alain");
    }
    }


    
}

