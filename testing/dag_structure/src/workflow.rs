use std::borrow::BorrowMut;
use std::{collections::HashMap, io::ErrorKind};
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Task {
    task_name: String,
    depends: Option<Vec<String>>, // indices of depending nodes
}

impl Task {
    pub fn new(task_name: &str, depends: Vec<String>) -> Self {
        Task {
            task_name: task_name.to_string(),
            depends: if depends.len() != 0 {
                Some(depends)
            } else {
                None
            },
        }
    }
}

#[derive(Debug, starlark::any::ProvidesStaticType, Default)]
pub struct Workflow(RefCell<HashMap<String, Task>>);


impl Workflow {

    pub fn add_nodes(&self, node: Task) -> Result<bool, ErrorKind> {

        if let Some(_) = self
            .0
            .borrow_mut()
            .insert(node.task_name.clone(), node)
        {
            Err(ErrorKind::AlreadyExists)
        } else {
            Ok(true)
        }
    }
}

#[test]
fn creating_workflow() {
    let task1 = Task::new("task-1", vec![]);
    let task2 = Task::new("task-2", vec![String::from("task-1")]);
    let task3 = Task::new("task-3", vec![String::from("task-1")]);
    let task4 = Task::new(
        "task-4",
        vec![String::from("task-2"), String::from("task-3")],
    );

    let mut wf = Workflow::default();
    wf.add_nodes(task1).unwrap();
    wf.add_nodes(task2).unwrap();
    wf.add_nodes(task3).unwrap();
    wf.add_nodes(task4).unwrap();
}
