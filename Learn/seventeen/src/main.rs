// Panic in Rust

fn main() {
    // panic!("Crash and burn!");

    // when panic call comes from a library because of a bug
    let v = vec![1, 2, 3];

    v[99];
}
