use std::io; 

fn main (){
    let mut age = String::new();

    println!("Enter age ");

    io::stdin()
        .read_line(&mut age)
        .expect("Failed");

    let age: u32 = match age.trim().parse(){
        Ok(a)=> a,
        Err(e) =>{
            println!("Error: {}", e);
            0
        }
    };

    println!{"You are {} years old", age};
}