use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

// Function to generate the Cargo.toml and main.rs files for the project
fn generate_cargo(
    project_name: &str,
    path: &PathBuf,
    main_file_content: &str,
    cargo_toml_content: &str,
) {
    // Generating the project using the "cargo new" command
    Command::new("cargo")
        .args(&["new", &project_name])
        .status()
        .unwrap();

    // Creating and writing into the files
    fs::write(&(path.join("src/main.rs")), main_file_content).unwrap();
    fs::write(&(path.join("Cargo.toml")), cargo_toml_content).unwrap();
}

// Function to generate the code for the main.rs file and the Cargo.toml file
fn generate_code(input: &str, depend_on: &str) -> Vec<String> {
    let content = format!(
        "\
fn main() {{
    println!(\"Hello, {}!\");
}}
",
        input
    );

    let dependencies = format!(
        "\
[package]
name = \"generated-project\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
{} = \"*\"
",
        depend_on
    );

    vec![content.to_string(), dependencies.to_string()]
}

fn main() {
    let project_name = String::from("generated-project");

    // Getting the current working directory
    let pwd = env::current_dir().unwrap();
    let proj_path = pwd.join(&project_name);
    
    // Example dependency
    let depend_on = String::from("serde_json");

    let content = generate_code("world", &depend_on);

    // Generating the project files
    generate_cargo(&project_name, &proj_path, &content[0], &content[1]);
}
