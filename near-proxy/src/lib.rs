use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, require, PanicOnDefault, Promise};

mod aurora;
mod utils;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    pub status_message_address: Vec<u8>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            status_message_address: vec![],
        }
    }

    #[private]
    pub fn set_status_message_address(&mut self, status_message_address: String) {
        require!(
            status_message_address.len() == 40,
            "Invalid status message address"
        );
        self.status_message_address = hex::decode(status_message_address).unwrap().to_vec();
    }

    pub fn set_message(&mut self, message: String) -> Promise {
        require!(
            self.status_message_address.len() == 20,
            "Status message address not set"
        );

        let mut contract = [0; 20];
        contract.copy_from_slice(&self.status_message_address);

        aurora::call(
            contract,
            [0; 32],
            utils::abi_encode(
                near_sdk::env::predecessor_account_id().as_str().to_string(),
                message,
            ),
        )
    }
}
