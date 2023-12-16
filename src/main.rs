use std::{fs::File, io};
fn main() {
    println!("Enter the file name");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Couldn't read the input");
    
    
}
