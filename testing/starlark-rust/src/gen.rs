// use super::*;

use starlark::environment::{Globals, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::Value;

pub fn say_hello() {
    let content: String = std::fs::read_to_string("hello_world.star").unwrap();

    // We first parse the content, giving a filename and the Starlark
    // `Dialect` we'd like to use (we pick standard).
    let ast: AstModule =
        AstModule::parse("hello_world.star", content.to_owned(), &Dialect::Standard).unwrap();

    // We create a `Globals`, defining the standard library functions available.
    // The `standard` function uses those defined in the Starlark specification.
    let globals: Globals = Globals::standard();
    // We create a `Module`, which stores the global variables for our calculation.
    let module: Module = Module::new();
    
    {
        // We create an evaluator, which controls how evaluation occurs.v
        let mut eval: Evaluator = Evaluator::new(&module);

        // And finally we evaluate the code using the evaluator.
        let _res: Value = eval.eval_module(ast, &globals).unwrap();
    }

    println!("{:?}", module.freeze().unwrap().get("output").unwrap());
}