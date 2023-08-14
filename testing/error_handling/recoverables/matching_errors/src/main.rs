use std::fs::File;
use std::io::ErrorKind;

fn main() {

    let test_file = File::open("hello.txt");

    let test_file_result= match test_file{

        Ok(file) => file,

        Err(error) => match error.kind() {

            ErrorKind::NotFound => match File::create("hello.txt"){

                Ok(fc) => fc,
                Err(e) => panic!("@@@ Problem in creating file {:?}", e)
            },
            
            other_error => panic!("###Some other problem in opening file: {:?}", other_error)
        },
    };

    println!("{:?}", test_file_result);

}
