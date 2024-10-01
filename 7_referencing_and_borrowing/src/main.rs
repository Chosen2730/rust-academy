fn main() {
    println!("Hello, world!");
    let mut a = String::from("value");
    let b = produce_string(&mut a);
    println!("a={a}");
}

fn produce_string(str: &mut String) {
    let new_a = str.push('a');
    new_a
}
