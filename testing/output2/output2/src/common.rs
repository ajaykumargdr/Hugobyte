
#![allow(unused_imports)]

use super::*;
#[derive(Debug,Flow)]
pub struct WorkflowGraph {
    edges: Vec<(usize, usize)>,
    nodes: Vec<Box<dyn Execute>>,
}

impl WorkflowGraph {
    pub fn new(size: usize) -> Self {
        WorkflowGraph {
            nodes: Vec::with_capacity(size),
            edges: Vec::new(),
        }
    }
}

#[macro_export]
macro_rules! impl_execute_trait {
    ($ ($struct : ty), *) => {
        
            paste!{
                $( impl Execute for $struct {
                    fn execute(&mut self) -> Result<(),String>{
        self.run()
    }

    fn get_task_output(&self) -> Value {
        self.output().clone().into()
    }

    fn set_output_to_task(&mut self, input: Value) {
        self.setter(input)
    }
                }
            )*
        }
    };
}

#[allow(dead_code, unused)]
pub fn join_hashmap<T: PartialEq + std::hash::Hash + Eq + Clone, U: Clone, V: Clone>(
    first: HashMap<T, U>,
    second: HashMap<T, V>,
) -> HashMap<T, (U, V)> {
    let mut data: HashMap<T, (U, V)> = HashMap::new();
    for (key, value) in first {
        for (s_key, s_value) in &second {
            if key.clone() == s_key.to_owned() {
                data.insert(key.clone(), (value.clone(), s_value.clone()));
            }
        }
    }
    data
}

