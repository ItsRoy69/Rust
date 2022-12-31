// Error checkin

use std::fs::File;
fn main() {
    let f = File::open("hello.txt").unwrap();

    // let foo = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file");
    //     }
    // };
}