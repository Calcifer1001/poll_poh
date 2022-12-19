use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, setup_alloc, AccountId};
use account_data::{AccountData};

setup_alloc!();

mod internal;
mod account_data;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PollContract {
    owner_id: AccountId,
    // Key is samsub id
    users_data: UnorderedMap<String, AccountData>,
}

#[near_bindgen]
impl PollContract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            users_data: UnorderedMap::new(b"users".to_vec()),
        }
    }

    pub fn add_account(&mut self, near_account_id: AccountId, samsub_id: String, is_valid: bool) {
        self.assert_owner();

        let account_data = AccountData::new(
            AccountId::from(&near_account_id),
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
        let keys = self.users_data.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, keys.len()))
            .map(|index: u64|
                // (&self.users_data.get(&keys.get(index).unwrap()).unwrap()).into()
                self.users_data.get(&keys.get(index).unwrap()).unwrap()
            )
            .collect()
    }

    pub fn edit_user_data(&mut self, samsub_id: String, is_valid: bool) -> bool {
        self.assert_owner();

        let user_data_option = self.users_data.get(&samsub_id);

        match user_data_option {
            Some(mut user_data) => {
                user_data.is_valid = is_valid;
                self.users_data.insert(&samsub_id, &user_data);
                true
            }
            None => false,
        }
    }
}