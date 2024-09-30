use std::io;
// Fahrenheit to Celsius: C = (F - 32) * 5/9
// Celsius to Fahrenheit: F = (C * 9/5) + 32
fn main() {
    println!("Hello, Simon; which conversion will you like to do? type C if celsius to fahrenheit, and F or fahrenheit to Celsius");
    let mut first_choice = String::new();
    io::stdin()
        .read_line(&mut first_choice)
        .expect("Error reading value");

    let first_choice = first_choice.trim();

    println!("Your first choice answer is {}", first_choice);
    if first_choice == "F" {
        println!("Enter a fahrenheit value");
        let mut second_input = String::new();
        io::stdin()
            .read_line(&mut second_input)
            .expect("Error reading value");

        let second_input: f64 = match second_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Please enter a number");
                return;
            }
        };
        let c: f64 = fahrenheit_to_celsius(second_input);
        println!("The converted value is {}°C", c);
    } else if first_choice == "C" {
        println!("Enter a value to conver to fahrenheit");
        let mut third_input = String::new();

        io::stdin()
            .read_line(&mut third_input)
            .expect("Error reading");

        let third_input: i32 = match third_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid value entered");
                return;
            }
        };

        let f: i32 = celsius_to_fahrenheit(third_input);
        println!("The converted value is {}°F", f);
    } else {
        println!("Invalid input, please enter a valid input");
    }
}
//  Celsius to Fahrenheit: F = (C * 9/5) + 32
fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    let result: i32 = (celsius * 9) + 32;
    result
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    let result: f64 = (fahrenheit - 32.0) * (5.0 / 9.0);
    result
}
