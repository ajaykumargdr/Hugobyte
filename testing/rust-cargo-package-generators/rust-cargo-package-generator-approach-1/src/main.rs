use std::process::Command; 
use std::{env, fs};

fn main() {

    let project_name = String::from("generated-project");

    // generating cargo
    Command::new("cargo")
        .args(&["new", &project_name])
        .status()
        .unwrap();

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
        
        // writing into generated code
        
        // getting current working directory
        let pwd = env::current_dir().unwrap();
        let proj_path = pwd.join(&project_name);
    
        // creating and writing into the files
        fs::write(&(proj_path.join("src/main.rs")),content).unwrap();
        fs::write(&(proj_path.join("Cargo.toml")),dependencies).unwrap();
}