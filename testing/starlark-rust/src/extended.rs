use super::*;

#[derive(Debug, Trace, Coerce, Display, ProvidesStaticType, NoSerialize, Allocative)]
#[repr(C)]
pub struct OneGen<T>{
    pub x : T,
}

starlark_complex_value!(pub One);

#[starlark_value(type = "One")]
impl<'v, T: ValueLike<'v> + 'v> StarlarkValue<'v> for OneGen<T>
    where Self: ProvidesStaticType<'v>,
{
    // To implement methods which work for both> `One` and `FrozenOne`,
    // use the `ValueLike` trait.

    // fn add(&self, rhs: Value<'v>, heap: &'v Heap) -> Option<anyhow::Result<Value<'v>>> {
    //     if let Some(rhs) = rhs.downcast_ref::<Self>() {
    //         Some(
    //             Ok(heap.alloc(One {
    //                 x: self.x,
    //                 }
    //             ))
    //         )
    //     } else {
    //         None
    //     }
    // }

}

impl<'v> Freeze for One<'v> {
    type Frozen = FrozenOne;
    fn freeze(self, freezer: &Freezer) -> anyhow::Result<Self::Frozen> {
        Ok(OneGen{x :self.x.freeze(freezer)?})
    }
}

// impl<'v T> One<'v>{
//     pub fn get(&self, x:T) -> anyhow::Result<One<'v>> {
//         Ok(
//             OneGen{ x }  
//         )
//     }
// }

#[starlark_module]
pub fn starlark_mdl(builder: &mut GlobalsBuilder) {

    fn create<'v>() -> anyhow::Result<One<'v>> {
        Ok(
            OneGen{ x: starlark::values::Value::new_bool(true)}  
        )
    }
}

#[starlark_module]
pub fn composer(builder: &mut GlobalsBuilder) {
    fn add(x: String) -> anyhow::Result<String> {
        Ok(String::from(x))
    }
}

pub fn extended_starlark() {
    let content: String = std::fs::read_to_string("extended_starlark_config/extended_config.star").unwrap();

    // We first parse the content, giving a filename and the Starlark
    // `Dialect` we'd like to use (we pick standard).
    let ast: AstModule =
        AstModule::parse("extended_config.star", content.to_owned(), &Dialect::Extended).unwrap();

    // We create a `Globals`, defining the standard library functions available.
    // The `standard` function uses those defined in the Starlark specification.
    use starlark::environment::LibraryExtension::*;
    let globals = GlobalsBuilder::extended_by( &[
        StructType,
        RecordType,
        EnumType,
        Map,
        Filter,
        Partial,
        ExperimentalRegex,
        Debug,
        Print,
        Pprint,
        Breakpoint,
        Json,
        Typing,
        Internal,
        CallStack,
    ])
    .with_struct("mdl", starlark_mdl)
    .with_struct("composer", composer)
    .build();
    // let globals = Globals::extended_internal();

    

    // We create a `Module`, which stores the global variables for our calculation.
    let module: Module = Module::new();
    {
        // module.set("starlark", starlark::values::structs::);
        // We create an evaluator, which controls how evaluation occurs.v
        let mut eval: Evaluator = Evaluator::new(&module);

        // eval.extra = Some(&starlark_mdl);
        // let x = structs::FrozenStructRef::from(value); 

        // And finally we evaluate the code using the evaluator.
        let res: Value = eval.eval_module(ast, &globals).unwrap();
        println!("{:?}", res);
    }
}
