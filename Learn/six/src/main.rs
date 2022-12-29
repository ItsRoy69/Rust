// Functions

fn main() {
    println!("Hello, world!");
    another_function();

    another_function2(5);

    let a = sum(5, 6);
    println!("The sum is: {}", a);

    let r = another_sum(4, 6);
    println!("The sum is: {}", r);
}


fn another_function() {
    println!("Another function.");
}

// Functions can take parameters
fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

// Functions can return values
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// Functions can return values
fn another_sum(c: i32, d: i32) -> i32 {
    let mut z = c + d;
    z = z * 100;

    return z;
}