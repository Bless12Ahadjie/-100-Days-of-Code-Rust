fn main() {
    let age: u32 = 20;
    let eligible:u32 = 17;

    if age > eligible{
        println!("You are eligible to vote");
    }
    else if age < eligible {
        println!(" You are not eligible to vote");
    }
    else {
        println!("You are an align");
    }

    lolo();
}

fn lolo() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}