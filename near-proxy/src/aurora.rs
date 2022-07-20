use borsh::BorshSerialize;
use near_sdk::{Gas, Promise};

type Address = [u8; 20];
type Wei = [u8; 32];

#[derive(BorshSerialize)]
struct CallArgs {
    /// Contract to be called
    pub contract: Address,
    /// Amount of wei to attach
    pub value: Wei,
    /// Input data to pass to the contract
    pub input: Vec<u8>,
}

impl CallArgs {
    fn serialize(&self) -> Vec<u8> {
        // Prepend byte one to signal enum version
        vec![vec![0], self.try_to_vec().unwrap()].concat()
    }
}

const AURORA_ACCOUNT_ID: &str = "aurora";
const CALL_GAS: Gas = Gas(20_000_000_000_000);

pub fn call(contract: Address, value: Wei, input: Vec<u8>) -> Promise {
    Promise::new(AURORA_ACCOUNT_ID.parse().unwrap()).function_call(
        "call".to_string(),
        CallArgs {
            contract,
            value,
            input,
        }
        .serialize(),
        0,
        CALL_GAS,
    )
}
