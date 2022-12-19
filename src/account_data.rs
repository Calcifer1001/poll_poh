use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountData {
    pub near_account_id: AccountId,
    pub samsub_id: String,
    pub is_valid: bool,
}

impl AccountData {
    pub fn new(near_account_id: AccountId, samsub_id: String, is_valid: bool) -> Self {
        Self {
            near_account_id,
            samsub_id,
            is_valid,
        }
    }
}