use super::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartypeInput {
    car_type: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, OpenWhisk)]
#[AuthKey = "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"]
#[ApiHost = "https://65.20.70.146:31001"]
#[Insecure = "true"]
#[Namespace = "guest"]
pub struct Cartype {
    action_name: String,
    pub input: CartypeInput,
    pub output: Value,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelavailInput {
    car_company_list: HashMap<String, Vec<String>>,

    company_name: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, OpenWhisk)]
#[AuthKey = "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"]
#[ApiHost = "https://65.20.70.146:31001"]
#[Insecure = "true"]
#[Namespace = "guest"]
pub struct Modelavail {
    action_name: String,
    pub input: ModelavailInput,
    pub output: Value,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelspriceInput {
    models: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, OpenWhisk)]
#[AuthKey = "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"]
#[ApiHost = "https://65.20.70.146:31001"]
#[Insecure = "true"]
#[Namespace = "guest"]
pub struct Modelsprice {
    action_name: String,
    pub input: ModelspriceInput,
    pub output: Value,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PurchaseInput {
    model_price_list: HashMap<String, i32>,
    model_name: String,
    price: i32,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, OpenWhisk)]
#[AuthKey = "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"]
#[ApiHost = "https://65.20.70.146:31001"]
#[Insecure = "true"]
#[Namespace = "guest"]
pub struct Purchase {
    action_name: String,
    pub input: PurchaseInput,
    pub output: Value,
}

impl_execute_trait!(Cartype, Modelavail, Modelsprice, Purchase);

impl Cartype {
    pub fn new(car_type: String, action_name: String) -> Self {
        Self {
            input: CartypeInput {
                car_type,
                ..Default::default()
            },
            action_name: action_name,
            ..Default::default()
        }
    }

    fn setter(&mut self, _value: Value) {}

    fn output(&self) -> Value {
        self.output.clone()
    }
}

impl Modelavail {
    pub fn new(company_name: String, action_name: String) -> Self {
        Self {
            input: ModelavailInput {
                company_name,
                ..Default::default()
            },
            action_name: action_name,
            ..Default::default()
        }
    }

    fn setter(&mut self, value: Value) {
        let value = value.get("car_company_list").unwrap();
        self.input.car_company_list = serde_json::from_value(value.clone()).unwrap();
    }

    fn output(&self) -> Value {
        self.output.clone()
    }
}

impl Modelsprice {
    pub fn new(action_name: String) -> Self {
        Self {
            input: ModelspriceInput {
                ..Default::default()
            },
            action_name: action_name,
            ..Default::default()
        }
    }

    fn setter(&mut self, value: Value) {
        let value = value.get("models").unwrap();
        self.input.models = serde_json::from_value(value.clone()).unwrap();
    }

    fn output(&self) -> Value {
        self.output.clone()
    }
}

impl Purchase {
    pub fn new(model_name: String, price: i32, action_name: String) -> Self {
        Self {
            input: PurchaseInput {
                model_name,
                price,
                ..Default::default()
            },
            action_name: action_name,
            ..Default::default()
        }
    }

    fn setter(&mut self, value: Value) {
        let value = value.get("model_price_list").unwrap();
        self.input.model_price_list = serde_json::from_value(value.clone()).unwrap();
    }

    fn output(&self) -> Value {
        self.output.clone()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub car_type: String,
    pub company_name: String,
    pub model_name: String,
    pub price: i32,
}