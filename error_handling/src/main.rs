use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("Problem opening the file: {:?}", error),
        },
    };

    // unwrap_or_else
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error)
    //         })
    //     }
    // })

    // expect or unwrap
    /*
    unwrap: panics if the value is None
    expect: returns an error if the value is None
    */
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("hello.txt should be found.");

    let value = read_from_file().unwrap_or_else(|error| {
        panic!("Problem opening the file: {:?}", error)
    });
    println!("{}", value);

    let value2 = read_from_file_with_closure().unwrap_or("".to_string());
    println!("{}", value2);

    let value3 = read_from_file_with_chain().unwrap_or("".to_string());
    println!("{}", value3);

}

fn read_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

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

fn read_from_file_with_closure() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_from_file_with_chain() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
