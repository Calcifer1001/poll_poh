use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountData {
    pub near_account_id: String,
    pub samsub_id: String,
    pub is_valid: bool,
}

impl Default for AccountData {
    fn default() -> Self {
        Self {
            near_account_id: String::from(""),
            samsub_id: String::from(""),
            is_valid: false,
        }
    }
}

impl AccountData {
    pub fn new(near_account_id: String, samsub_id: String, is_valid: bool) -> Self {
        Self {
            near_account_id,
            samsub_id,
            is_valid,
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContractData {
    owner_id: String,
    users_data: UnorderedMap<String, AccountData>,
}

impl Default for ContractData {
    fn default() -> Self {
        Self {
            owner_id: String::from(""),
            users_data: UnorderedMap::new(b"users".to_vec()),
        }
    }
}

impl From<&AccountData> for AccountData {
    fn from(account_data: &AccountData) -> Self {
        Self {
            near_account_id: account_data.near_account_id.clone(),
            samsub_id: account_data.samsub_id.clone(),
            is_valid: account_data.is_valid,
        }
    }
    
}

#[near_bindgen]
impl ContractData {

    pub fn new(&mut self) {
        assert!(
            self.owner_id == String::from(""),
            "Contract already has an owner id",
        );
        self.owner_id = String::from(env::signer_account_id());
    }

    pub fn add_account(&mut self, near_account_id: String, samsub_id: String, is_valid: bool) {
        assert!(
            env::signer_account_id() == self.owner_id,
            "Only owner can add account"
        );

        let account_data = AccountData::new(
            String::from(&near_account_id),
            String::from(&samsub_id),
            is_valid,
        );

        self.users_data.insert(&samsub_id, &account_data);

        env::log(
            format!(
                "User added successfully. Samsub id: {}, is_valid: {}, Near Account Id: {}",
                &samsub_id, &is_valid, &near_account_id
            )
            .as_bytes(),
        )
    }

    pub fn get_users_data(&self, from_index: u64, limit: u64) -> Vec<AccountData> {
    // pub fn get_users_data(&self, from: u64, limit: u64) -> Vec<(String, AccountData)> {
        let keys = self.users_data.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, keys.len()))
            .map(|index: u64|
                (&self.users_data.get(&keys.get(index).unwrap()).unwrap()).into()
            )
            .collect()

        // self.users_data.to_vec()
    }

    pub fn edit_user_data(&mut self, samsub_id: String, is_valid: bool) -> bool {
        assert!(
            env::signer_account_id() == self.owner_id,
            "Only owner can add account"
        );

        let user_data_option = self.users_data.get(&samsub_id);

        match user_data_option {
            Some(user_data) => {
                let acc_data = AccountData::new(
                    String::from(user_data.near_account_id),
                    String::from(&samsub_id),
                    is_valid,
                );
                self.users_data.insert(&samsub_id, &acc_data);
                // user_data.is_valid = is_valid;
                true
            }
            None => false,
        }
    }
}