// #![allow(unused)]

use clap::Parser;
use std::fs;

// *************
#[derive(Parser, Debug)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf    // ****************
    
    // PathBuf- similer to string but used to store file path
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError>{

    // will be bit hard to handle

    // let args:Vec<String> = env::args().collect();
    // println!("{:#?}", args);

//==============================================================
////////////////////////Using Clap /////////////////////////////
 
    // this will automatically maps the 
    // args to the struct variables

    // if you didn't passed the args it automatically throws error

    let args = Cli::parse();

    println!("pattern: {}\npath: {:?}\n", args.pattern, args.path);


    let file_content = fs::read_to_string(&args.path)
    .map_err( |err| CustomError(format!("Error reading `{:?}`: {}", args.path,err)))?;


    for line in file_content.lines(){

        if line.contains(&args.pattern){
            println!("{line}");
        }

    }

    
    Ok(())
}
