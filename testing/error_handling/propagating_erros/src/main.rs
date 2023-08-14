use std::fs::{self, File};
use std::io::{self, Read};

fn main() {

  println!("!!!1st {:?}", read_username_from_file());
  println!("!!!2nd {:?}", propagating_error_short());
  println!("!!!2nd {:?}", propagating_error_shortest());

}

fn read_username_from_file() -> Result<String, io::Error> {

    let username_file = File::open("username_file.txt");


    let mut username_file_result = match username_file{
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new(); 

    // File.read_to_string(&mut String)
    // let _test = username_file_result.read_to_string(&mut username);

    println!("Returned name is  {username}");


    // return Result<usize, Err>    // usize only has the size
    // we return Result<String, Err> // string has the name
    match username_file_result.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
    

    // username_file_result.read_to_string(&mut username)


}


fn propagating_error_short() -> Result<String, io::Error>{

    let mut username_file = File::open("username_file.txt")?;

    let mut username = String::new();

    // ?- return only if it gets error else it **continues**
    // Note : ? is only used where it may throughs Err (Err probably 'Error' type)
    // also can be used where you return Option<>
      
    username_file.read_to_string(&mut username)?;

    Ok(username)
}

fn propagating_error_shortest() -> Result<String, io::Error>{

    let mut username = String::new();

    File::open("username_file.txt")?.read_to_string(&mut username)?;

    Ok(username)
} 

// build in function for propagating
fn propagating_error_shortest_ever() -> Result<String, io::Error>{

    fs::read_to_string("username_file.txt")

}

