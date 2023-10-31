use super::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StakingpayoutInput {
    url: String,

    owner_key: String,

    address: String,

    era: u32,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Polkadot)]
#[Chain = "Westend"]
#[Operation = "stakingpayout"]
pub struct Stakingpayout {
    action_name: String,
    pub input: StakingpayoutInput,
    pub output: Value,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PushNotificationInput {
    token: String,

    message: Value,

    result: Option<H256>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, OpenWhisk)]
#[AuthKey = "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"]
#[ApiHost = "https://139.84.142.77:31001"]
#[Insecure = "true"]
#[Namespace = "guest"]

pub struct PushNotification {
    action_name: String,
    pub input: PushNotificationInput,
    pub output: Value,
}
impl_execute_trait!(Stakingpayout, PushNotification);
impl Stakingpayout {
    pub fn new(
        url: String,
        owner_key: String,
        address: String,
        era: u32,
        action_name: String,
    ) -> Self {
        Self {
            input: StakingpayoutInput {
                url,
                owner_key,
                address,
                era,
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

impl PushNotification {
    pub fn new(token: String, message: Value, action_name: String) -> Self {
        Self {
            input: PushNotificationInput {
                token,
                message,
                ..Default::default()
            },
            action_name: action_name,
            ..Default::default()
        }
    }

    fn setter(&mut self, value: Value) {
        let value = value.get("result").unwrap();
        self.input.result = serde_json::from_value(value.clone()).unwrap();
    }

    fn output(&self) -> Value {
        self.output.clone()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub url: String,
    pub owner_key: String,
    pub address: String,
    pub era: u32,
    pub token: String,
    pub message: Value,
}
