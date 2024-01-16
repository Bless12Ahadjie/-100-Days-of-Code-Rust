fn my_func() {
    let name: &str = "Bless";

    println!("Hey {name} this is your own defined function. HURRAY!!!");
}

// function with parameter
fn add(num1: u32, num2: u32) {
    let sum: u32 = num1 + num2;
    println! {"The addition is {sum}"};
}

//function with a return value
fn retu(a: i32, b: i32) -> i32 {
    let sum: i32 = a + b;
    sum
}

//main function
fn main() {
    println!("This return function returns {}", retu(1, 4));
    //   println!("This return function returns {hold}");

    add(4, 16);

    my_func();
}

//build a basic calcutator function
