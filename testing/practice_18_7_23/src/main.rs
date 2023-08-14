use random_::random_num::get_random_number;

// use std::fs::{self, File};
// use std::io::{self, ErrorKind, Read};
   
fn main() {

    println!("Got {}",get_random_number());

    // println!("{:?}", open_file(String::from("hello.txt")));
    // println!("{:?}", get_file_content(String::from("hello.txt")));
}

// fn open_file(file_name: String) -> Option<File>{

//     // Result<> :: Err
//     let file_ = File::open(file_name);    
    
//     match file_{
//         Ok(file) => Some(file),
//         Err(error) => match error.kind(){
//                 ErrorKind::NotFound => {
//                     println!("###Vanakamda maple!"); 
//                     None 
//                 },
//                 other => None
//             }
//     }
// }

// fn get_file_content(file_name: String) -> Result<String, io::Error> {

//     // let mut read_buffer = String::new();

//     // File::open(file_name)?.read_to_string(&mut read_buffer);

//     // Ok(read_buffer)

//     fs::read_to_string(file_name)

// }
