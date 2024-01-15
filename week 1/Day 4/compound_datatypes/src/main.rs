fn main() {
    //Tuples
    let tup = (500,4.5,1);
    let (a,_b,_c) = tup;
    println!("{a}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let one = x.2;

    println!("{one}");

    //array
    //declaring arrays
    let ar = [1,2,3,4];
    let say = ar[2];

    println!("index 2 holds {say}");

    let defed: [u8;3] = [3,5,9];

    let second = defed[2];
    println!("{second}");

    // initializing an array
    let kiss = [1; 4];
    let hold = kiss[2];

    println!("{hold}");
}
