// Program to calculate the area of a rectangle 

// Using struct in real life

// fn main() {
//     let width = 30;
//     let height = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width, height)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }


// A better way to do this is to use a struct

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
        let width = 30;
        let height = 50;

        let rect1 = Rectangle { width: 10, height: 20 };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }