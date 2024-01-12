fn main() {
    // ****Variables and Constants******
    let mut age = 20;
    const NAME:&str = "Bless";

    println!("{NAME} is now {age} years old");

    age = 26;

    println! ("{NAME} should be {age}  years old next 6 years");



    //*** Shadowing ****/
    let a = 5 ;
    let a = a + 1;

    {
        let a = a * 2;

        println! ("the inner scope is {a}");
    }

    println! ( "The outer scope becomes {a}");


    //*** datatypes ****/
    /* 
    with integers ypu can use i fo signed or u for unsigned and then specify the bit you wan to use say
    8,16,32,64 etc
     */

    let  y: u8 = 24;
    let  z: u32 = 2;

    println!("{y} {z}");

      /* 
        floats can be assigned using "F" folllowed by the but say f32 or f64
     */

    let flowt: f32 = 2.34;

    println! ("{flowt}");

          /* 
        boolean type using "bool"
     */

    let t: bool = true;

    println! ("{t}");

          /* 
        character type using "char"
     */

    let c = 'c';
    let z: char = 'ℤ'; 
    println!("{z} and {c}");



}
