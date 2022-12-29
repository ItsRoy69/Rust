// Loop

fn main() {
    loop {
        println!("Hello, world!");
        break;
    }

    // while loop

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    // for loop

    let a = [10, 20, 30, 40, 50];

    // iter is a method that returns each element in a collection
    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
