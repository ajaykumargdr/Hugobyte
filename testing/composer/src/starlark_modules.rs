use super::*;

#[starlark_module]
pub fn starlark_workflow_module(builder: &mut GlobalsBuilder) {
    fn task(
        kind: String,
        action_name: String,
        input_args: Value,
        attributes: Value,
        depend_on: Value,
        operation: Option<Value>,
    ) -> anyhow::Result<Task> {
        let input_args: Vec<Input> = serde_json::from_str(&input_args.to_json()?).unwrap();
        let attributes: HashMap<String, String> =
            serde_json::from_str(&attributes.to_json()?).unwrap();
        let depend_on: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(&depend_on.to_json()?).unwrap();

        let operation: Operation =  match operation{
            Some(op) => serde_json::from_str(&op.to_json()?).unwrap(),
            _ => Operation::Normal
        };

        Ok(Task {
            kind,
            action_name,
            input_args,
            attributes,
            operation,
            depend_on,
        })
    }

    fn workflows(
        name: String,
        version: String,
        tasks: Value,
        custom_types: Option<Value>,
        eval: &mut Evaluator,
    ) -> anyhow::Result<NoneType> {
        let tasks: Vec<Task> = serde_json::from_str(&tasks.to_json()?).unwrap();

        let custom_types: Option<Vec<String>> = match custom_types {
            Some(a) => serde_json::from_str(&a.to_json()?).unwrap(),
            None => None,
        };

        let mut task_hashmap = HashMap::new();

        for task in tasks {
            if task_hashmap.contains_key(&task.action_name) {
                return Err(Error::msg("Duplicate tasks, Task names must be unique"));
            } else {
                task_hashmap.insert(task.action_name.clone(), task);
            }
        }

        eval.extra
            .unwrap()
            .downcast_ref::<Composer>()
            .unwrap()
            .add_workflow(name, version, task_hashmap, custom_types)
            .unwrap();

        Ok(NoneType)
    }

    fn input_args(
        name: String,
        input_type: String,
        default_value: Option<String>,
    ) -> anyhow::Result<Input> {
        let default_value = match default_value {
            Some(b) => b,
            None => String::default(),
        };
        Ok(Input {
            name,
            input_type,
            default_value,
        })
    }

}

#[starlark_module]
pub fn starlark_datatype_module(builder: &mut GlobalsBuilder) {
    fn typ(name: String, fields: Value, eval: &mut Evaluator) -> anyhow::Result<String> {
        let fields: HashMap<String, String> = serde_json::from_str(&fields.to_json()?).unwrap();

        let composer = eval.extra.unwrap().downcast_ref::<Composer>().unwrap();

        let name = name.to_case(Case::Pascal);

        composer.add_custom_type(
            &name,
            format!(
                "make_input_struct!(\n\t{},\n\t{},\n\t[Default, Clone, Debug]\n);",
                &name,
                composer.parse_hashmap(&fields)
            ),
        );

        Ok(name)
    }

    fn string() -> anyhow::Result<String> {
        Ok("String".to_string())
    }

    fn bool() -> anyhow::Result<String> {
        Ok("String".to_string())
    }

    fn int(size: Option<i32>) -> anyhow::Result<String> {
        match size {
            Some(x) => match x {
                8 | 16 | 32 | 64 | 128 => Ok(format!("i{}", x)),
                _ => Err(Error::msg("Size is invalid")),
            },
            None => Ok("i32".to_string()),
        }
    }

    fn hashmap(field1: String, field2: String) -> anyhow::Result<String> {
        Ok(format!("HashMap<{}, {}>", field1, field2))
    }

    fn list(field1: String) -> anyhow::Result<String> {
        Ok(format!("Vec<{}>", field1))
    }

    fn Struct(type_name: String, eval: &mut Evaluator) -> anyhow::Result<String> {
        let composer = eval.extra.unwrap().downcast_ref::<Composer>().unwrap();
        let type_name = type_name.to_case(Case::Pascal);

        if !type_name.is_empty() && composer.custom_types.borrow().contains_key(&type_name) {
            Ok(type_name)
        } else {
            Err(Error::msg("type {type_name} does not exist"))
        }
    }
}

#[starlark_module]
pub fn starlark_operation_module(builder: &mut GlobalsBuilder) {

    fn normal() -> anyhow::Result<Operation>{
        Ok(Operation::Normal)
    }

    fn cat() -> anyhow::Result<Operation>{
        Ok(Operation::Cat)
    }

    fn map(field: String) -> anyhow::Result<Operation>{
        Ok(Operation::Map(field))
    }
}