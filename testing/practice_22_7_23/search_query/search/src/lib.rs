use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config{
    query: String,
    file_path: String
}

impl Config {
    pub fn build(args:&[String] ) -> Result<Config, &'static str>{
        if args.len() >= 3{
            Ok(Config{
                    query: args[1].clone(),
                    file_path: args[2].clone()
                })
        } else{
            Err("needed arguments are not provided")
        }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{

    let contents = fs::read_to_string(config.file_path.clone())?;

    for line in search(&config.query, &contents){
        println!("{line}")
    }    

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec: Vec<&str> = Vec::new(); 

    for line in contents.lines(){
        if line.contains(query){
            vec.push(line);
        }
    }
    vec
}
