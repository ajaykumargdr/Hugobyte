use anyhow;
use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::starlark_module;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::{none::NoneType, Value};
use std::collections::HashMap;

mod workflow;
use workflow::*;

#[starlark_module]
pub fn starlark_workflow(builder: &mut GlobalsBuilder) {
    fn task(
        kind: String,
        action_name: String,
        input: Value,
        attributes: Value,
        deps: Value,
        eval: &mut Evaluator,
    ) -> anyhow::Result<NoneType> {
        let input: HashMap<String, String> = serde_json::from_str(&input.to_json()?).unwrap();
        let attributes: HashMap<String, String> =
            serde_json::from_str(&attributes.to_json()?).unwrap();
        let deps: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(&deps.to_json()?).unwrap();

        eval.extra
            .unwrap()
            .downcast_ref::<Workflow>()
            .unwrap()
            .add_nodes(Task::new(&kind, &action_name, input, attributes, deps))
            .unwrap();

        Ok(NoneType)
    }
}

pub fn starlark_create_workflow(store: &mut Workflow, config_file: &str) {
    let content: String = std::fs::read_to_string(config_file).unwrap();

    let ast = AstModule::parse(config_file, content.to_owned(), &Dialect::Standard).unwrap();
    // We build our globals adding some functions we wrote
    let mut globals = GlobalsBuilder::new().with(starlark_workflow).build();
    let mut module = Module::new();
    {
        let mut eval = Evaluator::new(&mut module);
        // We add a reference to our store
        eval.extra = Some(store);
        eval.eval_module(ast, &mut globals).unwrap();
    }
}

fn main() {}

#[test]
fn get_flow_of_workflow_pass() {
    let mut store = Workflow::default();
    starlark_create_workflow(&mut store, "example.star");

    let flow_1 = vec!["action-1", "action-3", "action-5", "action-2", "action-4"];
    let flow_2 = vec!["action-1", "action-5", "action-2", "action-3", "action-4"];
    let flow_3 = vec!["action-5", "action-1", "action-2", "action-3", "action-4"];
    let flow_4 = vec!["action-5", "action-1", "action-3", "action-2", "action-4"];

    let possibilities = [flow_1, flow_2, flow_3, flow_4];

    let flow: Vec<String> = store.get_flow();

    let mut pass = false;

    for p in possibilities.iter() {
        if p.eq(&flow) {
            pass = true
        }
    }

    assert!(pass);
}
