use std::collections::HashMap;

macro_rules! make_struct {
    ($name:literal, [$($field_name:literal: $field_type:literal),*]) => {
        paste::paste!{
            struct [<$name>] {
                $( [<$field_name>]: [<$field_type>], )*
            }
        }
    };
}

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
pub struct Workflow(Vec<Task>);

impl Workflow {
    pub fn add_task(&mut self, task: Task) {
        self.0.push(task);
    }

    fn execute_task(task: &Task) {
        println!("task {:?} is executed", task.action_name);
    }

    pub fn execute_workflow(&self) {
        for task in self.0.iter() {
            Workflow::execute_task(task);
        }
    }
}


fn main() {
    let mut w1 = Workflow::default();

    // load_flows(&mut w1);


    w1.execute_workflow()
}
