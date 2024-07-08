// Find all our documentation at https://docs.near.org
use near_sdk::{env, log, near, AccountId, Gas, NearToken, PanicOnDefault, Promise, PromiseError};

pub mod external;
pub use crate::external::*;

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    // v2.multichain-mpc.testnet
    pub mpc_account: AccountId,
}

#[near]
impl Contract {
    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    pub fn init(mpc_account: AccountId) -> Self {
        // assert!(!env::state_exists(), "Already initialized");
        Self { mpc_account }
    }

    // Public - query external greeting
    pub fn request_sign(&self, payload: [u8; 32], path: String) -> Promise {
        // let _sign_request: SignRequest = SignRequest {
        //     key_version: 0,
        //     payload,
        //     path,
        // };
        // Create a promise to call MPC.sign()
        // let promise = mpc_sign::ext(self.mpc_account.clone())
        //     .with_static_gas(GAS_FOR_SIGN_CALL)
        //     .sign(_sign_request);

        let args = serde_json::json!({ "key_version": 0, "payload": payload, "path": path })
            .to_string()
            .into_bytes();

        let promise = Promise::new(self.mpc_account.clone()).function_call(
            "sign".to_owned(),
            args,
            NearToken::from_near(0),
            GAS_FOR_SIGN_CALL,
        );

        return promise.then(
            // Create a promise to callback mpc_sign_callback
            Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(5))
                .mpc_sign_callback(),
        );
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn mpc_sign_callback(
        &self,
        #[callback_result] call_result: Result<SignResult, PromiseError>,
    ) -> SignResult {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            log!("There was an error contacting v2.multichain-mpc.testnet");
            return SignResult {
                big_r: "none".to_owned(),
                s: "none".to_owned(),
            };
        }

        // Return the greeting
        let result: SignResult = call_result.unwrap();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const MPC_SIGN: &str = "v2.multichain-mpc.testnet";

    #[test]
    fn initializes() {
        let mpc_account: AccountId = MPC_SIGN.parse().unwrap();
        let contract = Contract::init(mpc_account);
        assert_eq!(contract.mpc_account, MPC_SIGN)
    }
}
