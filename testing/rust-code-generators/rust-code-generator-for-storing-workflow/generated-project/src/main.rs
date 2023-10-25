#[derive(Debug, Clone, Default)]
struct Flow {
    field_1: String,
    field_2: i32,
    field_3: f64,
    field_4: bool,
}

#[derive(Debug, Clone, Default)]
struct WorkFlow(Vec<Flow>);

impl WorkFlow {
    pub fn add_flow(&mut self, flow: Flow) {
        self.0.push(Flow {
            ..flow
        });
    }
}

fn main() {
    let mut w1 = WorkFlow::default();

    // Loading flows into the workflow
    load_flows(&mut w1);

    println!("Hello!! world {:?}", w1);
}

fn load_flows(workflow: &mut WorkFlow) {
	workflow.add_flow(Flow {
                field_1: "flow-1".to_string(),
                field_2: 1,
                field_3: 0.1,
                field_4: true,
            });
	workflow.add_flow(Flow {
                field_1: "flow-2".to_string(),
                field_2: 2,
                field_3: 0.2,
                field_4: false,
            });
}