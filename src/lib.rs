use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, near_bindgen, PanicOnDefault, Timestamp};
use near_sdk::serde::{Serialize, Serializer};
use near_sdk::collections::Vector;
use near_sdk::serde::ser::SerializeStruct;

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct TemperatureHistory {
    time: Timestamp,
    value: f32,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
enum Relay {
    ON,
    OFF,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
enum StepMotor {
    ON(i8),
    OFF,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    temperature: Vector<TemperatureHistory>,
    relay_1: Relay,
    relay_2: Relay,
    step_motor: StepMotor,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_name: String) -> Self {
        Self {
            owner: owner_name.parse().unwrap(),
            temperature: Vector::new(b"m".to_vec()),
            relay_1: Relay::OFF,
            relay_2: Relay::OFF,
            step_motor: StepMotor::OFF,
        }
    }

    pub fn add_temperature(&mut self, new_value: f32) {
        assert_eq!(env::predecessor_account_id(), self.owner);
        let value = TemperatureHistory { time: env::block_timestamp_ms(), value: new_value };
        self.temperature.push(&value);
    }

    pub fn change_relay_1_status(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner);
        match &self.relay_1 {
            Relay::ON => self.relay_1 = Relay::OFF,
            Relay::OFF => self.relay_1 = Relay::ON,
        };
    }

    pub fn change_relay_2_status(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner);
        match &self.relay_2 {
            Relay::ON => self.relay_2 = Relay::OFF,
            Relay::OFF => self.relay_2 = Relay::ON,
        };
    }

    pub fn add_step_motor_task(&mut self, task: i8) {
        assert_eq!(env::predecessor_account_id(), self.owner);

        match &self.step_motor {
            StepMotor::ON(_) => env::panic_str("Step Motor is busy"),
            StepMotor::OFF => self.step_motor = StepMotor::ON(task),
        };
    }

    pub fn execute_step_motor_task(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner);
        self.step_motor = StepMotor::OFF
    }

    pub fn get_state(&self) -> &Self {
        return self;
    }
}

impl Serialize for Contract {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut contract = serializer.serialize_struct("Contract", 4)?;
        contract.serialize_field("relay_1", &self.relay_1).unwrap();
        contract.serialize_field("relay_2", &self.relay_2).unwrap();
        contract.serialize_field("step_motor", &self.step_motor).unwrap();
        contract.serialize_field::<Vec<TemperatureHistory>>("temperature", &self.temperature.to_vec()).unwrap();
        contract.end()
    }
}