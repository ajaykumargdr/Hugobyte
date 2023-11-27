// #[macro_use]
// extern crate starlark;

use allocative::Allocative;
use anyhow;
use serde_derive::{Deserialize, Serialize};
use starlark::environment::{FrozenModule, Globals, GlobalsBuilder, Module};
use starlark::eval::{Evaluator, ReturnFileLoader};
use starlark::syntax::{AstModule, Dialect, DialectTypes};
use starlark::values::{
    none::NoneType, Heap, NoSerialize, ProvidesStaticType, StarlarkValue, Value, ValueError,
    ValueLike, structs
};

use starlark::values::{ComplexValue, Coerce, Freezer, FrozenValue, Trace, Tracer, Freeze};

use starlark::{starlark_module, starlark_simple_value, starlark_complex_value, starlark_complex_values};
use std::cell::RefCell;
// use std::fmt::{self, Display};
use derive_more::Display;
use starlark_derive::starlark_value;

use starlark::environment::LibraryExtension;
mod gen;
use gen::*;
mod extended;
use extended::*;

/*
pub mod basics {
    use super::*;

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

        // We create an evaluator, which controls how evaluation occurs.v
        let mut eval: Evaluator = Evaluator::new(&module);

        // And finally we evaluate the code using the evaluator.
        let res: Value = eval.eval_module(ast, &globals).unwrap();

        println!("{:?}", res);
    }

    // This defines the function that is visible to Starlark
    #[starlark_module]
    pub fn starlark_quadratic(builder: &mut GlobalsBuilder) {
        fn quadratic(a: i32, b: i32, c: i32, x: i32) -> anyhow::Result<i32> {
            Ok(a * x * x + b * x + c)
        }
    }

    pub fn function_call() {
        // We build our globals to make the function available in Starlark
        let globals = GlobalsBuilder::new().with(starlark_quadratic).build();
        let module = Module::new();
        let mut eval = Evaluator::new(&module);

        let starlark_code: String = std::fs::read_to_string("function-call.star").unwrap();

        let ast = AstModule::parse(
            "function-call.star",
            starlark_code.to_owned(),
            &Dialect::Standard,
        )
        .unwrap();
        let res = eval.eval_module(ast, &globals).unwrap();

        println!("{:?}", res.unpack_int()); // Verify that we got an `int` return value of 4 * 8^2 + 2 * 8 + 1 = 273
    }

    // throws `trait AllocValue not implemented` Error
    /*

    #[derive(Debug, Clone)]
    struct Flow{
       hook_type: String,
       flow_type: String,
       task_name: String,
       depend_on: String
    }

    impl Flow{
       pub fn new(hook_type:&str, flow_type:&str, task_name:&str, depend_on:&str) -> Flow{
          Flow{
             hook_type: String::from(hook_type),
             flow_type: String::from(flow_type),
             task_name: String::from(task_name),
             depend_on: String::from(depend_on)
          }
       }
    }

    #[starlark_module]
    pub fn starlark_create_flow(builder: &mut GlobalsBuilder) {
          fn create_flow(hook_type:&str, flow_type:&str, task_name:&str, depend_on:&str) -> anyhow::Result<Flow> {
             Ok(Flow::new(hook_type, flow_type, task_name, depend_on).clone())
          }
    }

    fn creating_flow_by_function_call(){

       // We build our globals to make the function available in Starlark
       let globals = GlobalsBuilder::new().with(starlark_create_flow).build();
       let module = Module::new();
       let mut eval = Evaluator::new(&module);

       // Let's test calling the function from Starlark code
       let starlark_code:String = std::fs::read_to_string("create-flow.star").unwrap();

       let ast = AstModule::parse("create-flow.star", starlark_code.to_owned(), &Dialect::Standard).unwrap();
       let res = eval.eval_module(ast, &globals).unwrap();

       println!("{:?}",res.unpack_int()); // Verify that we got an `int` return value of 4 * 8^2 + 2 * 8 + 1 = 273

    }

    */

    // Using Starlark as an enhanced JSON

    // Define a store in which to accumulate JSON strings
    #[derive(Debug, starlark::any::ProvidesStaticType, Default)]
    struct Store(RefCell<Vec<String>>);

    impl Store {
        fn add(&self, x: String) {
            self.0.borrow_mut().push(x)
        }
    }

    #[starlark_module]
    fn starlark_emit(builder: &mut GlobalsBuilder) {
        fn emit(x: starlark::values::Value, eval: &mut Evaluator) -> anyhow::Result<NoneType> {
            // We modify extra (which we know is a Store) and add the JSON of the
            // value the user gave.
            eval.extra
                .unwrap()
                .downcast_ref::<Store>()
                .unwrap()
                .add(x.to_json()?);
            println!("------> {:?}", x);
            Ok(NoneType)
        }
    }

    fn using_starlark_as_json() {
        let content: String = std::fs::read_to_string("json.star").unwrap();

        let ast = AstModule::parse("json.star", content.to_owned(), &Dialect::Standard).unwrap();
        // We build our globals adding some functions we wrote
        let globals = GlobalsBuilder::new().with(starlark_emit).build();
        let module = Module::new();
        let store = Store::default();
        {
            let mut eval = Evaluator::new(&module);
            // We add a reference to our store
            eval.extra = Some(&store);
            eval.eval_module(ast, &globals).unwrap();
        }
        assert_eq!(&*store.0.borrow(), &["1", "[\"test\"]", "{\"x\":\"y\"}"]);

        println!("{:?}", store.0);
    }

    // Enable the load statement

    // Get the file contents (for the demo), in reality use `AstModule::parse_file`.
    pub fn get_source_file(file: &str) -> String {
        std::fs::read_to_string(format!("ab-starlark-files/{file}")).unwrap()
    }

    pub fn get_module(file: &str) -> anyhow::Result<FrozenModule> {
        let ast = AstModule::parse(file, get_source_file(file).to_owned(), &Dialect::Standard)?;

        // We can get the *loaded modules* from `ast.loads`.
        // And ultimately produce a `loader` capable of giving those modules to Starlark.
        let mut loads: Vec<(String, FrozenModule)> = Vec::new();

        println!("{:?}", ast.loads());

        for load in ast.loads() {
            loads.push((load.module_id.to_owned(), get_module(load.module_id)?));
        }

        let modules = loads.iter().map(|(a, b)| (a.as_str(), b)).collect();

        println!("{:?}", modules);

        let mut loader = ReturnFileLoader { modules: &modules };

        let globals = Globals::standard();
        let module = Module::new();
        {
            let mut eval = Evaluator::new(&module);
            eval.set_loader(&mut loader);
            eval.eval_module(ast, &globals)?;
        }
        // module refers to a module that cannot be modified
        // After creating a module we freeze it, preventing further mutation.
        // It can now be used as the input for other Starlark modules.
        Ok(module.freeze()?) //
    }

    pub fn using_type_annotation() {
        let content: String = std::fs::read_to_string("type-annotation.star").unwrap();

        // Make the dialect enable types
        let dialect = Dialect {
            enable_types: DialectTypes::Enable,
            ..Dialect::Standard
        };
        // We could equally have done `dialect = Dialect::Extended`.
        let ast = AstModule::parse("type-annotation.star", content.to_owned(), &dialect).unwrap();
        let globals = Globals::standard();
        let module = Module::new();
        let mut eval = Evaluator::new(&module);
        let res = eval.eval_module(ast, &globals);
        // We expect this to fail, since it is a type violation
        // assert!(res.unwrap_err().to_string().contains("Value `test` of type `string` does not match the type annotation `int`"));
        res.unwrap();
    }

    pub fn enable_load_statement() {
        let ab = get_module("ab.star").unwrap();

        println!("{:?}", ab);

        // assert_eq!(ab.get("ab").unwrap().unpack_int(), Some(21));
        // print!("{:?}", ab);

        println!("{:?}", ab.get("ab").unwrap());
        println!("{:?}", ab.get("ba").unwrap());
    }
}
*/

/*
pub mod mid {
    use super::*;

    // Define a store in which to accumulate JSON strings

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
        fn add(&self, hook_type: String, flow_type: String, task_name: String, depend_on: String) {
            self.0.borrow_mut().push(Flow {
                hook_type,
                flow_type,
                task_name,
                depend_on,
            })
        }
    }
    /* uncomment this to test `using function in starlark`
        #[starlark_module]
        pub fn starlark_workflow(builder: &mut GlobalsBuilder) {
            fn create_flow(
                hook_type: Value,
                flow_type: Value,
                task_name: Value,
                depend_on: Value,
                eval: &mut Evaluator,
            ) -> anyhow::Result<NoneType> {
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

        pub fn starlark_create_workflow() {
            let content: String = std::fs::read_to_string("map-ds-star-files/example.star").unwrap();

            let ast = AstModule::parse(
                "map-ds-star-files/example.star",
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

        println!("{:#?}", store.0.borrow());

        }
    */
    //////////////////////////////////////////////

    // Define complex numbers
    #[derive(
        Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative, Serialize, Deserialize,
    )]
    struct Complex {
        real: i32,
        imaginary: i32,
    }
    starlark_simple_value!(Complex);

    impl Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imaginary)
        }
    }
    /*////////////////////////////////////////////////////////////////
        impl AllocValue for Complex {
            fn alloc_value(self) -> Value {
                Value::dict([
                    ("name", Value::string(self.name)),
                    ("age", Value::int(self.age)),
                ])
            }
        }
    *////////////////////////////////////////////////////////////////

    impl<'v> StarlarkValue<'v> for Complex {

        // How we add them
        fn add(&self, rhs: Value<'v>, heap: &'v Heap) -> Option<anyhow::Result<Value<'v>>> {
            if let Some(rhs) = rhs.downcast_ref::<Self>() {
                Some(Ok(heap.alloc(Complex {
                    real: self.real + rhs.real,
                    imaginary: self.imaginary + rhs.imaginary,
                })))
            } else {
                None
            }
        }
    }

    #[starlark_module]
    pub fn starlark_workflow(builder: &mut GlobalsBuilder) {
        fn create_complex() -> anyhow::Result<Complex> {
            Ok(Complex {
                real: 5,
                imaginary: 5,
            })
        }

        fn create_complex_2(cmplx: Value) -> anyhow::Result<Complex> {
            let input: Complex = serde_json::from_str(&cmplx.to_json()?).unwrap();

            Ok(input)
        }
    }

    pub fn using_rust_types_in_starlark() {
        let content: String = std::fs::read_to_string("using_rust_types.star").unwrap();

        let ast = AstModule::parse(
            "using_rust_types.star",
            content.to_owned(),
            &Dialect::Standard,
        )
        .unwrap();
        let globals = GlobalsBuilder::new().with(starlark_workflow).build();
        let module = Module::new();
        // We inject some complex numbers into the module before we start.
        // let a = module.heap().alloc(Complex {real: 1, imaginary: 8});

        // module.set("a", a);

        // let b = module.heap().alloc(Complex {real: 4, imaginary: 2});

        // module.set("b", b);

        let mut eval = Evaluator::new(&module);
        let res = eval.eval_module(ast, &globals).unwrap();
        // assert_eq!(res.unpack_str(), Some("5 + 10i"));

        println!("{:?}", res);
    }
}
*/

// use basics::*;
// use mid::*;

fn test_function() {
    let content: String = std::fs::read_to_string("test.star").unwrap();

    // We first parse the content, giving a filename and the Starlark
    // `Dialect` we'd like to use (we pick standard).
    let ast: AstModule =
        AstModule::parse("test.star", content.to_owned(), &Dialect::Extended).unwrap();

    // We create a `Globals`, defining the standard library functions available.
    // The `standard` function uses those defined in the Starlark specification.
    // let globals = GlobalsBuilder::new().with(starlark_mdl).build();

    let globals = Globals::extended_internal();

    // We create a `Module`, which stores the global variables for our calculation.
    let module: Module = Module::new();
    {
        // module.set("starlark", starlark::values::structs::);
        // We create an evaluator, which controls how evaluation occurs.v
        let mut eval: Evaluator = Evaluator::new(&module);

        // let x = structs::FrozenStructRef::from(value); 

        // And finally we evaluate the code using the evaluator.
        let res: Value = eval.eval_module(ast, &globals).unwrap();
        println!("{:?}", res);
    }
    // let x:struct = 

    
}



fn main() {
    // say_hello();
    // function_call();
    // using_starlark_as_json();
    // using_type_annotation();
    // enable_load_statement();
    // gen::starlark_create_workflow();
    // using_rust_types_in_starlark();
    // gen::say_hello();

    // gen::function_call();
    // mid::using_rust_types_in_starlark();
    // gen::say_hello();

    // test_function();

    extended::extended_starlark();
}

