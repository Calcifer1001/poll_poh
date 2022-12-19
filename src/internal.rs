use crate::PollContract;
use near_sdk::{env};

impl PollContract {
    pub fn assert_owner(&self) {
        assert!(
            env::predecessor_account_id() == self.owner_id,
            "Only owner can call"
        );
    }
}