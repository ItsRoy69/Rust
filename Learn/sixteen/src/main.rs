// Patterns

fn main() {
    let name = String::from("Rust");

    println!("Character at position 3 is {}", 
    match name.chars().nth(3){
        Some(c) => c.to_string(),
        None => "No character found".to_string(),
    }
    )
}

