use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn generate_cargo(
    project_name: &str,
    path: &PathBuf,
    main_file_content: &str,
    cargo_toml_content: &str,
) {
    // generating cargo
    Command::new("cargo")
        .args(&["new", &project_name])
        .status()
        .unwrap();

    // creating and writing into the files
    fs::write(&(path.join("src/main.rs")), main_file_content).unwrap();
    fs::write(&(path.join("Cargo.toml")), cargo_toml_content).unwrap();
}

struct Flow {
    field_1: String,
    field_2: i32,
    field_3: f64,
    field_4: bool,
}

// loading flows
fn generate_load_flow_function(flows: Vec<Flow>) -> String {
    let mut content = String::new();

    for flow in flows.iter() {
        content = format!("{}\tworkflow.add_flow( Flow{{ field_1: \"{}\".to_string(), field_2: {}, field_3: {}, field_4: {}}});\n",content, flow.field_1, flow.field_2, flow.field_3, flow.field_4);
    }

    format!("fn load_flows(workflow: &mut WorkFlow){{\n{}}}", content)
}

fn generate_main_file_code() -> String {
    let flow1 = Flow {
        field_1: String::from("flow-1"),
        field_2: 1,
        field_3: 0.1,
        field_4: true,
    };

    let flow2 = Flow {
        field_1: String::from("flow-2"),
        field_2: 2,
        field_3: 0.2,
        field_4: false,
    };

    let load_flow_function = generate_load_flow_function(vec![flow1, flow2]);

    let main_file = format!(
        "\
#[derive(Debug, Clone, Default)]
struct Flow {{
    field_1: String,
    field_2: i32,
    field_3: f64,
    field_4: bool,
}}

#[derive(Debug, Clone, Default)]
struct WorkFlow(Vec<Flow>);

impl WorkFlow {{
    pub fn add_flow(&mut self, flow: Flow) {{
        self.0.push(
            Flow{{
                ..flow
            }}
        );
    }}
}}

fn main() {{

    let mut w1 = WorkFlow::default();

    //loading flows into the workflow 
    load_flows(&mut w1);

    println!(\"Hello!! world {{:?}}\", w1);
}}

{}",
        load_flow_function
    );

    main_file
}

fn generate_code() -> Vec<String> {
    let dependencies = "\
[package]
name = \"generated-project\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
# some pre-determined dependencies
";

    vec![generate_main_file_code(), dependencies.to_string()]
}

fn main() {
    let project_name = String::from("generated-project");

    // getting current working directory
    let pwd = env::current_dir().unwrap();
    let proj_path = pwd.join(&project_name);

    let content = generate_code();

    generate_cargo(&project_name, &proj_path, &content[0], &content[1]);
}
