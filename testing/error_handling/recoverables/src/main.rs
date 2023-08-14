use std::fs::File;

fn main() {

    let test_file = File::open("hello.txt");    //  return Result<file, Err>

    // Panics 

    let test_file_result = match &test_file{
        Result::Ok(file) => file,
        Result::Err(error) => panic!("@@@Problem in opening the file: {:?}", error),
    };
    
    // only Result::Err message (not panics)
    println!("End of program {:?}", test_file);

// Shortcut to simply through the error 

    // let test_file = File::open("hello2.txt").unwrap();

    // throws panic message (provided msg)
    let test_file = File::open("hello2.txt").expect("***hello2.txt should be included in this project which is not found now***");


}
