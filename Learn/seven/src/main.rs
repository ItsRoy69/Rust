// branches - if else

fn main() {

    let a= 15;

    if a < 10 {
       println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    if a % 2 == 0 {
        println!("Number is divisible by 2");
    } else if a % 3 == 0 {
        println!("Number is divisible by 3");
    }
    else {
        println!("None");
    }
}
