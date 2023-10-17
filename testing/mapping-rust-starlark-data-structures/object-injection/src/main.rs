use allocative::Allocative;
use starlark::environment::{Globals, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::{NoSerialize, ProvidesStaticType, StarlarkValue};
use starlark::{starlark_simple_value, starlark_type};
use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative)]
pub struct Flow {
    hook_type: String,
    flow_type: String,
    task_name: String,
    depend_on: String,
}

starlark_simple_value!(Flow);

impl Display for Flow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Flow{{hook_type: {}, flow_type: {}, task_name: {}, depend_on: {}}}",
            self.hook_type, self.flow_type, self.task_name, self.depend_on
        )
    }
}

impl<'v> StarlarkValue<'v> for Flow {
    starlark_type!("Flow");
}

fn main() {
    let content: String = std::fs::read_to_string("example.star").unwrap();

    let ast = AstModule::parse(
        "example.star",
        content.to_owned(),
        &Dialect::Standard,
    )
    .unwrap();
		// Create a new globals environment
    let globals = Globals::standard();
		// Create a new module
    let module = Module::new();

    // Create two Flow objects and allocate them on the heap
    let a = module.heap().alloc(Flow {
        hook_type: "flow-1".to_string(),
        flow_type: "type of the flow 1".to_string(),
        task_name: "name of the task 1".to_string(),
        depend_on: "dependencies 1".to_string(),
    });

    let b = module.heap().alloc(Flow {
        hook_type: "flow-2".to_string(),
        flow_type: "type of the flow 2".to_string(),
        task_name: "name of the task 2".to_string(),
        depend_on: "dependencies 2".to_string(),
    });

    // Inject the Flow objects into the module
    module.set("a", a);
    module.set("b", b);

		// Create a new evaluator for the module
    let mut eval = Evaluator::new(&module);

    // Evaluate the module using the globals environment
    let res = eval.eval_module(ast, &globals).unwrap();

		// Print the result
    println!("{:?}", res);
}