fn main() {
    let age: u32 = 20;
    let eligible:u16 = 18;

    if age => eligible{
        println!("You are eligible to vote");
    }
    else if age < eligible {
        println!(" You are not eligible to vote");
    }
    else {
        println!("You are an align");
    }
}