use std::io;
fn main() {
    // println!("Hello, world!");
    // let a: f64 = 0.1;
    // let b = 0.2;
    // let c = a + b;
    // let d: u32 = 1;
    // let f: i16 = -40

    // let c = 'z';
    // let z: char = 'ℤ';
    // let heart_eyed_cat = '😻';

    // let tup: (char, i32, f64) = ('a', -34, 45.6);
    // let (x, y, z) = tup;

    // let months: [&str; 12] = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];

    // println!("{}", months[12]);

    // println!("{:?}", y);
    // println!("{}", tup.0);

    // println!("{:?}", a + b);
    // println!("{:?}", c)

    let numbers: [u16; 5] = [2, 5, 2, 5, 7];
    println!("Enter a number");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Error reading");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let ele = numbers[index];
    println!("The element at index {} is {}", index, ele);
}
