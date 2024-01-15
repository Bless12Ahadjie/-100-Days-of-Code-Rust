
fn my_func (){
    let name: &str = "Bless";

    println!("Hey {name} this is your own defined function. HURRAY!!!");
}

// function with parameter
fn add (num1:u32 , num2: u32){
  let sum = num1 + num2;
  println! {"The addition is {sum}"};
}

//main function
fn main() {
   add(4,16);
//    my_func();
}
