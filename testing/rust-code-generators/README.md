# Rust Code Generator Module
This `README.md` file provides an overview of the Rust Code Generator Module, which offers two distinct approaches for generating code within Rust projects.

### Approach 1: Generating Code by Injecting Text Format

The first approach demonstrates how to inject code into a project using text format. It involves creating a new Cargo package with customizable `main.rs` and `Cargo.toml` files. The process involves the following steps:

Use the `generate_cargo` function to generate a new Cargo package by calling the `cargo new` command.
Specify the desired code for the `main.rs` file and the dependencies for the `Cargo.toml` file.
Use the `fs::write` function to create and write into the respective files.

By following this approach, you can generate and inject code dynamically into your Rust project.

### Approach 2: Generating Code with Workflow Storage
The second approach focuses on generating code for storing a workflow. 

It includes the following components:

* The `Flow` struct, which represents a workflow with fields such as `field_1, field_2, field_3` and `field_4`.
* The `generate_load_flow_function` function, which generates code for loading flows into a WorkFlow struct.
* The `generate_main_file_code` function, which creates the main file code that initializes a WorkFlow instance, loads flows, and prints the result.
* The `generate_code` function, which generates the `main` file code and the dependencies for the `Cargo.toml` file.