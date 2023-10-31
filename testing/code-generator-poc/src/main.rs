use anyhow;
use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::starlark_module;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::{none::NoneType, Value};
use std::collections::HashMap;

mod workflow;

use workflow::*;
// use generator::*;

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

pub fn create_workflow(store: &mut Workflow, config_file: &str) {
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

fn main() {
    let mut workflow = Workflow::default();

    create_workflow(&mut workflow, "example.star");

    // workflow.generate();

    println!("{:?}", workflow.get_common_inputs() );

}

#[test]
fn get_flow_of_workflow_pass() {
    let mut store = Workflow::default();
    create_workflow(&mut store, "example.star");

    let flow_1 = vec!["Action1", "Action3", "Action5", "Action2", "Action4"];
    let flow_2 = vec!["Action1", "Action5", "Action2", "Action3", "Action4"];
    let flow_3 = vec!["Action5", "Action1", "Action2", "Action3", "Action4"];
    let flow_4 = vec!["Action5", "Action1", "Action3", "Action2", "Action4"];

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


//////////////////////////////////////////////
