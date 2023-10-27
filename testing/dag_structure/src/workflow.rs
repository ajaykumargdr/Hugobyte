use std::cell::RefCell;
use std::path::PathBuf;
use std::process::Command;
use std::{collections::HashMap, io::ErrorKind};
use std::{env, fs};

#[derive(Debug, Clone)]
pub struct Task {
    kind: String,
    action_name: String,
    pub input: HashMap<String, String>,
    pub attributes: HashMap<String, String>,
    deps: HashMap<String, HashMap<String, String>>,
}

impl Task {
    pub fn new(
        kind: &str,
        action_name: &str,
        input: HashMap<String, String>,
        attributes: HashMap<String, String>,
        deps: HashMap<String, HashMap<String, String>>,
    ) -> Self {
        Task {
            kind: kind.to_string(),
            action_name: action_name.to_string(),
            input,
            attributes,
            deps,
        }
    }
}

#[derive(Debug, starlark::any::ProvidesStaticType, Default, Clone)]
pub struct Workflow(RefCell<HashMap<String, Task>>);

impl Workflow {
    pub fn add_nodes(&self, node: Task) -> Result<bool, ErrorKind> {
        if let Some(_) = self.0.borrow_mut().insert(node.action_name.clone(), node) {
            Err(ErrorKind::AlreadyExists)
        } else {
            Ok(true)
        }
    }

    fn get_dependencies(&self, task_name: &str) -> Option<Vec<String>> {
        let mut deps = Vec::<String>::new();

        for d in self.0.borrow().get(task_name).unwrap().deps.iter() {
            deps.push(d.0.clone());
        }

        Some(deps)
    }

    fn dfs(&self, task_name: &str, visited: &mut HashMap<String, bool>, flow: &mut Vec<String>) {
        visited.insert(String::from(task_name), true);

        for d in self.get_dependencies(task_name).unwrap().iter() {
            if !visited[d] {
                self.dfs(d, visited, flow);
            }
        }

        flow.push(String::from(task_name));
    }

    pub fn get_flow(&self) -> Vec<String> {
        let mut visited = HashMap::<String, bool>::new();
        let mut flow = Vec::<String>::new();

        for t in self.0.borrow().iter() {
            visited.insert(String::from(t.0), false);
        }

        for t in self.0.borrow().iter() {
            if !visited[t.0] {
                self.dfs(t.0, &mut visited, &mut flow)
            }
        }

        flow
    }

    pub fn get_task(&self, task_name: &str) -> Task {
        self.0.borrow().get(task_name).unwrap().clone()
    }

    pub fn get_task_input_data(&self, task_name: &str, task: &HashMap<String, String>) -> String {
        let mut input = format!("{task_name}Input, [");

        for (i, field) in task.iter().enumerate() {
            input = format!("{input}{}:{}", field.0, field.1);

            if i != task.len() - 1 {
                input = format!("{input},");
            } else {
                input = format!("{input}]");
            }
        }

        input
    }

    // Function to generate the code for the main.rs file
    fn generate_main_file_code(&self) -> String {
        let mut main_file = format!(
            "\
macro_rules! make_struct {{
    ($name:ident, [$($field_name:ident: $field_type:ident),*]) => {{
        // converting literals into identifiers
            struct $name {{
                $( $field_name: $field_type, )*
            }}
    }};
}}
"
        );

        for (task_name, task) in self.0.borrow().iter() {
            let input_data = self.get_task_input_data(task_name, &task.input);
            main_file = format!("{main_file}\nmake_struct!({input_data});");
        }

        main_file = format!(
            "{main_file}\n\n\
fn main() {{}}

#[test]
fn generated_structs_test() {{
    let a1 = Action1Input {{
        url: \"http\".to_string(),
        era: 1,
        address: \"aurras\".to_string(),
        owner_key: \"ow234bdn234ciouwndfuwbfo456wefc\".to_string(),
    }};

    let a2 = Action2Input {{
        url: \"http\".to_string(),
        era: 1,
        address: \"aurras\".to_string(),
        owner_key: \"ow234bdn234ciouwndfuwbfo456wefc\".to_string(),
    }};

    let a3 = Action3Input {{
        url: \"http\".to_string(),
        era: 1,
        address: \"aurras\".to_string(),
        owner_key: \"ow234bdn234ciouwndfuwbfo456wefc\".to_string(),
    }};

    let a4 = Action4Input {{
        url: \"http\".to_string(),
        era: 1,
        address: \"aurras\".to_string(),
        owner_key: \"ow234bdn234ciouwndfuwbfo456wefc\".to_string(),
    }};

    let a5 = Action5Input {{
        url: \"http\".to_string(),
        era: 1,
        address: \"aurras\".to_string(),
        owner_key: \"ow234bdn234ciouwndfuwbfo456wefc\".to_string(),
    }};
}}
        
"
        );

        main_file
    }

    // returns main file and dependencies file
    fn get_code(&self) -> Vec<String> {
        let dependencies = "\
[package]
name = \"generated-project\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
";

        vec![self.generate_main_file_code(), dependencies.to_string()]
    }

    // Function to generate a new Cargo package and write the main.rs and Cargo.toml files
    fn generate_cargo(
        &self,
        project_name: &str,
        path: &PathBuf,
        main_file_content: &str,
        cargo_toml_content: &str,
    ) {
        // Generating a new Cargo package
        Command::new("cargo")
            .args(&["new", &project_name])
            .status()
            .unwrap();

        // Creating and writing into the files
        fs::write(&(path.join("src/main.rs")), main_file_content).unwrap();
        fs::write(&(path.join("Cargo.toml")), cargo_toml_content).unwrap();
    }

    pub fn generate(&self) {
        let project_name = String::from("generated-project");
        // Getting the current working directory
        let pwd = env::current_dir().unwrap();
        let proj_path = pwd.join(&project_name);

        let content = self.get_code();

        // Generating the Cargo package and writing the main.rs and Cargo.toml files
        self.generate_cargo(&project_name, &proj_path, &content[0], &content[1]);
    }
}
