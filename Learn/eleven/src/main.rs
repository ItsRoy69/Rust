// Slices

fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];

    println!("hello contains = {}", hello);
}
