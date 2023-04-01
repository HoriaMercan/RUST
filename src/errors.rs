// TODO 1 - fix this function
pub fn divide(a: i32, b: i32) -> Option<isize> {
    if b == 0 {
        None
    } else {
        Some((a / b) as isize)
    }
}

// TODO 3 - fix this function
pub fn divide_error(a: i32, b: i32) -> Result<isize, String> {
    match divide(a, b) {
        None => Err(String::from("De ce imparti la 0?")),
        Some(c) => Ok(c)
    }
}

// TODO 5 - list files in the diretcory
//  - if the directory does not exist, return None
use std::{fs, path::Path};
use std::vec::Vec;

fn list_dir(path: &str) -> Option<Vec<String>>{

    if Path::new(path).exists() {

        let paths = fs::read_dir(path).unwrap();

        let mut vector = Vec::new(); 

        for __path in paths {
            vector.push(__path.unwrap().path().as_path().display().to_string());
        }

        Some(vector)

    } else {
        None
    }
}

pub fn run() {
    // TODO 2 - make the print work, use match and/or if let
    // println!("division: ", divide(a, b));
    match divide(32, 3) {
        None => println!("Don't divide by 0"),
        Some(c) => println!("division result : {}", c),
    }
    

    // TODO 4 - make the print work, use match and/or if let
    // println!("division: ", divide_error(a, b));
    let new_div = divide_error(10, 0);
    match new_div {
        Ok(c) => println!("division: {}", c),
        Err(err) => println!("{}", err),
    }

    // TODO 6 - use the list_directory function to print the current directory
    match list_dir("./") {
        None => println!("Error"),
        Some(vector) => {
            for element in vector {
                println!("{}", element);
            }
        }
    }

}
