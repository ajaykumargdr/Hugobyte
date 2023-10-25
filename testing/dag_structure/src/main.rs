// #[macro_use]
// extern crate starlark;

use anyhow;
use starlark::environment::{FrozenModule, Globals, GlobalsBuilder, Module};
use starlark::eval::{Evaluator, ReturnFileLoader};
use starlark::syntax::{AstModule, Dialect, DialectTypes};
use starlark::values::{none::NoneType, Value, Heap, StarlarkValue, ValueError, ValueLike, ProvidesStaticType, NoSerialize};
use starlark::{
    // starlark_type,
    // starlark_simple_value, 
    starlark_module};
// use std::cell::RefCell;
// use std::fmt::{self, Display};
// use allocative::Allocative;

mod workflow;
use workflow::*;

#[starlark_module]
pub fn starlark_workflow(builder: &mut GlobalsBuilder) {

    fn add_task(
        task_name: String,
        depend_on: Value,
        eval: &mut Evaluator,
    ) -> anyhow::Result<NoneType> {
        
        let depend_on: Vec<String> = serde_json::from_str(&depend_on.to_json()?).unwrap();

        eval.extra.unwrap().downcast_ref::<Workflow>().unwrap().add_nodes(
            Task::new(
                &task_name,
                depend_on,
            )
        ).unwrap();

        Ok(NoneType)
    }

}

pub fn starlark_create_workflow() {
    let content: String = std::fs::read_to_string("example.star").unwrap();

    let ast = AstModule::parse("example.star", content.to_owned(), &Dialect::Standard).unwrap();
    // We build our globals adding some functions we wrote
    let mut globals = GlobalsBuilder::new().with(starlark_workflow).build();
    let mut module = Module::new();
    let mut store = Workflow::default();
    {
        let mut eval = Evaluator::new(&mut module);
        // We add a reference to our store
        eval.extra = Some(&mut store);
        eval.eval_module(ast, &mut globals).unwrap();
    }

    // prints the workflow
    println!("{:#?}", store);   
}

fn main() {
    starlark_create_workflow();
}
