fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
    produce_string(s1);
    // println!("{s1}");
}

fn produce_string(str: String) {
    println!("{str}",);
}
