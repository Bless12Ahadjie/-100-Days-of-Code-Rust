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
}
