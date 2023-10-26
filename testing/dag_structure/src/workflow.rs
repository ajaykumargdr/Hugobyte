use std::cell::RefCell;
use std::{collections::HashMap, io::ErrorKind};

#[derive(Debug, Clone)]
pub struct Task {
    kind: String,
    action_name: String,
    input: HashMap<String, String>,
    attributes: HashMap<String, String>,
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

#[derive(Debug, starlark::any::ProvidesStaticType, Default)]
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
}
