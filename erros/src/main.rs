use std::fs::File;
use std::io;
use std::io::Read;
fn main() {
   let get_file = File::open("teste.txt");

   let _result = match get_file {
    Ok(value) => value,
    Err(err) => match err.kind() {
        ErrorKind::NotFound => match File::Create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file {:?}",e)
        },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error);
        }
    },
   };
}

fn exameple1() {
    let get_file = File::open("teste.txt")
        .expect("hello.txt should be included in this project");
}

fn example2() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => Ok(file),
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn example3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
