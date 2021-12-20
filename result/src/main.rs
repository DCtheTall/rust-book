use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

// `main` can return a Result<T, E> type
// `Box<dyn Error>` essentially just means "any error type"
fn main() -> Result<(), Box<dyn Error>> {
    // f is Result<std::fs::File, std::io::Error>
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening file {:?}", error),
    // };

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            },
        },
    };

    // Nested `match` is verbose, instead use closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });

    // You can also just unwrap() the value of the Result and panic if it fails
    let f = File::open("hello.txt").unwrap();

    // You can also provide an error message with expect()
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // Can use ? in main if it returns a Result
    let f = File::open("hello.txt")?;
    Ok(())
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let f = File::open(path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Can be more concise with the ? operator
fn read_file2(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// An even more concise rewrite using method chaining after ?
fn read_file3(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

// Even shorter rewrite using fs::read_to_string
fn read_file4(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

// Compiler error, can only use ? when function returns a Result
// fn read_file_error(path: &str) {
//     let f = File::open(path)?;
// }
