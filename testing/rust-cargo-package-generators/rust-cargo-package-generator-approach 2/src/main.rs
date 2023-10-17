use std::env;
use std::fs::{write, create_dir_all};

fn main() {

    let content ="\
pub fn message() -> &'static str {
    \"Hello, World!\"
}

fn main(){
    println!(\"{}\", message());
}
"; 

    let dependencies = "\
[package]
name = \"generated-project\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
# some pre-determined dependencies";

    // code generation part
    
    // getting current working directory
    let pwd = env::current_dir().unwrap();

    // creating new directory in the same directory where the Cargo.toml is located 
    create_dir_all(pwd.join("./generated_project/src")).unwrap();

    let proj_path = pwd.join("./generated_project");

    // creating and writing into the files
    write(&(proj_path.join("src/main.rs")),content).unwrap();
    write(&(proj_path.join("Cargo.toml")),dependencies).unwrap();
}
