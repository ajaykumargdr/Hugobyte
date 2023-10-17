use anyhow;
use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::none::NoneType;
use starlark::starlark_module;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Flow {
    hook_type: String,
    flow_type: String,
    task_name: String,
    depend_on: String,
}

#[derive(Debug, starlark::any::ProvidesStaticType, Default)]
pub struct WorkFlow(RefCell<Vec<Flow>>);

impl WorkFlow {
		// Adds a new Flow to the vector
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
pub fn starlark_workflow(builder: &mut GlobalsBuilder) {
    
		// Define a Starlark function to create a Flow 
		// and add it to the Workflow
		fn create_flow(
        hook_type: String,
        flow_type: String,
        task_name: String,
        depend_on: String,
        eval: &mut Evaluator,
    ) -> anyhow::Result<NoneType> {
        // We modify extra (which we know is a Store) and add the JSON of the
        // value the user gave.
        eval.extra
            .unwrap()
            .downcast_ref::<WorkFlow>()
            .unwrap()
            .add(hook_type, flow_type, task_name, depend_on);

        Ok(NoneType)
    }
}

fn main() {
		// Read the content of "example.star" file
    let content: String = std::fs::read_to_string("example.star").unwrap();

		// Parse the content into an AST module
    let ast = AstModule::parse(
        "example.star",
        content.to_owned(),
        &Dialect::Standard,
    )
    .unwrap();

		// Build the globals environment and add the starlark_workflow module
    let globals = GlobalsBuilder::new().with(starlark_workflow).build();
    let module = Module::new();
    let store = WorkFlow::default();

    {
        let mut eval = Evaluator::new(&module);
        // Store a reference to the Workflow in eval.extra
        eval.extra = Some(&store);
				// Evaluate the module using the globals environment
        eval.eval_module(ast, &globals).unwrap();
    }

    // Print the vector of Flows stored in the Workflow
    println!("{:#?}", store.0.borrow());
}
