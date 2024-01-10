#![no_std]
#![no_main]

extern crate alloc;

mod balances;
pub mod constants;
pub mod entry_points;
mod error;
mod utils;

use core::str::FromStr;

use alloc::{borrow::ToOwned, string::String};

use balances::{get_balances_uref, read_balance_from, transfer_balance, write_balance_to};
use entry_points::generate_entry_points;
use serde::{Deserialize, Serialize};

use casper_contract::{
    contract_api::{
        runtime::{self, get_caller, get_key, get_named_arg, revert},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, system::CallStackElement, AsymmetricType, Key, PublicKey, RuntimeArgs, U256,
};

use constants::{ALLOWANCES, BALANCES, INIT_ENTRY_POINT_NAME, MESSAGE, NAME, SYMBOL, TOTAL_SUPPLY};
pub use error::Cep18Error;

use utils::{get_total_supply_uref, read_total_supply_from, write_total_supply_to};

#[derive(Serialize, Deserialize)]
struct Message {
    p: String,
    op: String,
    tick: String,
    max: Option<String>,
    lim: Option<String>,
    amt: Option<String>,
    to: Option<String>,
    decimals: Option<String>,
}

#[no_mangle]
pub extern "C" fn inscribe() {
    let message: String = runtime::get_named_arg(MESSAGE);

    let metadata: Message = serde_json_wasm::from_str::<Message>(&message)
        .map_err(|_| Cep18Error::MyError)
        .unwrap_or_revert();

    if metadata.op == *"deploy" {
        deploy(metadata)
    } else if metadata.op == *"mint" {
        mint(metadata)
    } else if metadata.op == *"transfer" {
        transfer(metadata)
    } else {
        revert(Cep18Error::MyError1);
    }
}

fn mint(message: Message) {
    // mint
    // {
    //     "p": "cbrc-20",
    //     "op": "mint",
    //     "tick": "demo",
    //     "amt": "1000"
    //   }

    let owner = runtime::get_caller();
    let amt = message.amt.unwrap();
    let amount: U256 = U256::from_str(&amt).unwrap();

    let balances_uref = get_balances_uref();
    let total_supply_uref = get_total_supply_uref();
    let new_balance = {
        let balance = read_balance_from(balances_uref, Key::from(owner));
        balance
            .checked_add(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    let new_total_supply = {
        let total_supply: U256 = read_total_supply_from(total_supply_uref);
        total_supply
            .checked_add(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    write_balance_to(balances_uref, Key::from(owner), new_balance);
    write_total_supply_to(total_supply_uref, new_total_supply);
}

fn transfer(message: Message) {
    // transfer
    // {
    //     "p": "cbrc-20",
    //     "op": "transfer",
    //     "tick": "demo",
    //     "amt": "100"
    //     "to": "{A valid Casper Network public key}"
    //   }
    let amt = message.amt.unwrap();
    let recipient_str = message.to.unwrap();
    let recipient = PublicKey::from_hex(recipient_str)
        .unwrap()
        .to_account_hash();

    let sender = utils::get_immediate_caller_address().unwrap_or_revert();

    if sender == Key::from(recipient) {
        revert(Cep18Error::CannotTargetSelfUser);
    }
    let amount = U256::from_str(&amt).unwrap();

    transfer_balance(sender, Key::from(recipient), amount).unwrap_or_revert();
}

fn deploy(message: Message) {
    // deploy
    // {
    //     "p": "cbrc-20",
    //     "op": "deploy",
    //     "tick": "demo",
    //     "max": "21000000",
    //     "lim": "1000"
    //     "decimals":"18"
    //   }
    let name = message.p;
    let symbol = message.tick;
    let max = message.max.unwrap();
    let lim = message.lim.unwrap();
    let decimals = message.decimals.unwrap();

    // Call contract to initialize it
    let init_args: RuntimeArgs =
        runtime_args! {"max" => max, NAME => name,SYMBOL=>symbol,"lim"=>lim,"decimals"=>decimals};

    let contract_package = match runtime::get_call_stack()
        .iter()
        .nth_back(0)
        .unwrap_or_revert()
        .to_owned()
    {
        CallStackElement::StoredContract {
            contract_package_hash,
            ..
        } => contract_package_hash,
        _ => revert(Cep18Error::MyError2),
    };

    runtime::call_versioned_contract::<()>(
        contract_package,
        None,
        INIT_ENTRY_POINT_NAME,
        init_args,
    );
}
/// Initiates the contracts states. Only used by the installer call,
/// later calls will cause it to revert.
#[no_mangle]
pub extern "C" fn init() {
    if get_key(ALLOWANCES).is_some() {
        revert(Cep18Error::AlreadyInitialized);
    }

    let symbol = get_named_arg::<String>("symbol");
    runtime::put_key(SYMBOL, storage::new_uref(symbol).into());
    let max = get_named_arg::<String>("max");
    runtime::put_key(
        TOTAL_SUPPLY,
        storage::new_uref(U256::from_str(&max).unwrap()).into(),
    );
    let lim = get_named_arg::<String>("lim");
    runtime::put_key("lim", storage::new_uref(lim).into());
    let decimals = get_named_arg::<String>("decimals");
    runtime::put_key("decimals", storage::new_uref(decimals).into());

    storage::new_dictionary(ALLOWANCES).unwrap_or_revert();
    let balances_uref = storage::new_dictionary(BALANCES).unwrap_or_revert();
    let initial_supply = U256::from_str(&max).unwrap();
    let caller = get_caller();
    write_balance_to(balances_uref, caller.into(), initial_supply);
}
pub fn install_contract() {
    let entry_points = generate_entry_points();

    let (contract_hash, _contract_version) =
        storage::new_contract(entry_points, None, Some("csprs_package".to_owned()), None);

    // Store contract_hash and contract_version under the keys CONTRACT_NAME and CONTRACT_VERSION
    runtime::put_key("csprs_contract", contract_hash.into());
}

#[no_mangle]
pub extern "C" fn call() {
    install_contract()
}
