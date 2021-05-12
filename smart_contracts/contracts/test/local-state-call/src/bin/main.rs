#![no_std]
#![no_main]

extern crate alloc;

use core::str::FromStr;

use alloc::string::{String, ToString};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::FromBytes, CLTyped, ContractHash, RuntimeArgs, URef};
use local_state::{DEFAULT_LOCAL_KEY_NAME, DEFAULT_LOCAL_KEY_VALUE};
use local_state_call::{
    Operation, ARG_CONTRACT_HASH, ARG_FORGED_UREF, ARG_OPERATION, ARG_SHARE_UREF_ENTRYPOINT,
    NEW_LOCAL_KEY_NAME, NEW_LOCAL_KEY_VALUE,
};

/// Calls local state contract by hash as passed by `ARG_CONTRACT_HASH` argument and returns a
/// single value.
fn call_local_state_contract<T: CLTyped + FromBytes>(entrypoint: &str) -> T {
    let contract_hash: ContractHash = runtime::get_named_arg(ARG_CONTRACT_HASH);
    runtime::call_contract(contract_hash, entrypoint, RuntimeArgs::default())
}

#[no_mangle]
pub extern "C" fn call() {
    let operation = {
        let arg_operation: String = runtime::get_named_arg(ARG_OPERATION);
        Operation::from_str(&arg_operation).unwrap_or_revert()
    };

    match operation {
        Operation::Write => {
            let entrypoint: String = runtime::get_named_arg(ARG_SHARE_UREF_ENTRYPOINT);
            let uref = call_local_state_contract(&entrypoint);
            let value: String = NEW_LOCAL_KEY_VALUE.to_string();
            storage::write_local(uref, NEW_LOCAL_KEY_NAME, value);
        }
        Operation::Read => {
            let entrypoint: String = runtime::get_named_arg(ARG_SHARE_UREF_ENTRYPOINT);
            let uref = call_local_state_contract(&entrypoint);
            let maybe_value = storage::read_local(uref, DEFAULT_LOCAL_KEY_NAME).unwrap_or_revert();
            // Whether the value exists or not we're mostly interested in validation of access
            // rights
            let value: String = maybe_value.unwrap_or_default();
            assert_eq!(value, DEFAULT_LOCAL_KEY_VALUE);
        }
        Operation::ForgedURef => {
            let uref: URef = runtime::get_named_arg(ARG_FORGED_UREF);
            let value: String = NEW_LOCAL_KEY_VALUE.to_string();
            storage::write_local(uref, NEW_LOCAL_KEY_NAME, value);
        }
    }
}
