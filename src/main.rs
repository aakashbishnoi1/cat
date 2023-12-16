use std::{fs::File, io::{self, Read}};
use io::ErrorKind;
fn main() {
    println!("Enter the file name");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Couldn't read the input");
    let filename = filename.trim();

    let target_file = File::open(filename);
    let target_file = match target_file {
        Ok(file) => file,
        Err(error) => match error.kind()  {
                ErrorKind::NotFound => panic!("Error: {}", error),
                other_error => panic!("Problem opening the file {}", other_error),

        },
    };

}
