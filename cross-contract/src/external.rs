// Find all our documentation at https://docs.near.org
use near_sdk::ext_contract;
use near_sdk::{near, Gas, Promise};

pub const GAS_FOR_SIGN_CALL: Gas = Gas::from_tgas(250);

#[near(serializers=[borsh, json])]
pub struct SignRequest {
    pub payload: [u8; 32],
    pub path: String,
    pub key_version: u32,
}

#[near(serializers=[borsh, json])]
pub struct SignResult {
    pub big_r: String,
    pub s: String,
}

// https://github.com/near/mpc/blob/develop/chain-signatures/contract/src/lib.rs#L153
#[ext_contract(mpc_sign)]
trait MpcSign {
    fn sign(&mut self, request: SignRequest) -> Promise;
}
