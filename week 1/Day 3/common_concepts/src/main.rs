fn main() {
    // Variables and Constants
    // let mut age = 20;
    // const NAME:&str = "Bless";

    // println!("{NAME} is now {age} years old");

    // age = 26;

    // println! ("{NAME} should be {age}  years old next 6 years");
    let x = 5 ;
    let x = x + 1;

    {
        let x = x * 2;

        println! ("the inner scope is {x}")
    }

    println! ( "The outer scope becomes {x}")
}
