#[macro_use]
extern crate starlark;

use anyhow;
use starlark::any::ProvidesStaticType;
use starlark::environment::{ GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::{none::NoneType, Value};
use std::cell::RefCell;
use std::env;
use std::fs;

fn generate_cargo() {
    let pwd = env::current_dir().unwrap();

    fs::create_dir_all(pwd.join("../generated_project/src")).unwrap();

    let proj_path = pwd.join("../generated_project");

    fs::write(
        &(proj_path.join("src/main.rs")),"\
pub fn message() -> &'static str {
    \"Hello, World!\"
}

fn main(){
    println!(\"{}\", message());
}
"
).unwrap();

    fs::write(
        &(proj_path.join("Cargo.toml")),
"[package]
name = \"build_scripts\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
# some predetermined dependencies
"
    ).unwrap();

    // println!("cargo:rerun-if-changed=build.rs");
}

#[derive(Debug, Clone)]
pub struct Flow {
    hook_type: String,
    flow_type: String,
    task_name: String,
    depend_on: String,
}

#[derive(Debug, ProvidesStaticType, Default)]
struct WorkFlow(RefCell<Vec<Flow>>);

impl WorkFlow {
    fn add(&self, hook_type: String, flow_type: String, task_name: String, depend_on: String) {
        self.0.borrow_mut().push(Flow {
            hook_type,
            flow_type,
            task_name,
            depend_on,
        })
    }
}

#[starlark_module]
fn starlark_workflow(builder: &mut GlobalsBuilder) {
    fn create_flow(
        hook_type: Value,
        flow_type: Value,
        task_name: Value,
        depend_on: Value,
        eval: &mut Evaluator,
    ) -> anyhow::Result<NoneType> {
        // We modify extra (which we know is a Store) and add the JSON of the
        // value the user gave.
        eval.extra.unwrap().downcast_ref::<WorkFlow>().unwrap().add(
            hook_type.to_json()?.replace("\"", ""),
            flow_type.to_json()?.replace("\"", ""),
            task_name.to_json()?.replace("\"", ""),
            depend_on.to_json()?.replace("\"", ""),
        );
        // println!("------> {:?}\n", x);

        println!("Type -> {:?}", hook_type.get_type_starlark_repr());
        Ok(NoneType)
    }
}

fn starlark_create_workflow() {
    let content: String = std::fs::read_to_string("example.star").unwrap();

    let ast = AstModule::parse(
        "/example.star",
        content.to_owned(),
        &Dialect::Standard,
    )
    .unwrap();
    // We build our globals adding some functions we wrote
    let globals = GlobalsBuilder::new().with(starlark_workflow).build();
    let module = Module::new();
    let store = WorkFlow::default();
    {
        let mut eval = Evaluator::new(&module);
        // We add a reference to our store
        eval.extra = Some(&store);
        eval.eval_module(ast, &globals).unwrap();
    }

    println!("{:#?}", store.0);
}

fn main() {

    starlark_create_workflow();

}
