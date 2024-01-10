//! Implementation details.
use core::convert::TryInto;

use alloc::{vec, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    system::CallStackElement,
    ApiError, CLTyped, Key, URef, U256,
};

use crate::{constants::TOTAL_SUPPLY, error::Cep18Error};

/// Gets [`URef`] under a name.
pub(crate) fn get_uref(name: &str) -> URef {
    let key = runtime::get_key(name)
        .ok_or(ApiError::MissingKey)
        .unwrap_or_revert();
    key.try_into().unwrap_or_revert()
}

/// Returns address based on a [`CallStackElement`].
///
/// For `Session` and `StoredSession` variants it will return account hash, and for `StoredContract`
/// case it will use contract package hash as the address.
fn call_stack_element_to_address(call_stack_element: CallStackElement) -> Key {
    match call_stack_element {
        CallStackElement::Session { account_hash } => Key::from(account_hash),
        CallStackElement::StoredSession { account_hash, .. } => {
            // Stored session code acts in account's context, so if stored session wants to interact
            // with an CEP-18 token caller's address will be used.
            Key::from(account_hash)
        }
        CallStackElement::StoredContract {
            contract_package_hash,
            ..
        } => Key::from(contract_package_hash),
    }
}

/// Gets the immediate session caller of the current execution.
///
/// This function ensures that Contracts can participate and no middleman (contract) acts for users.
pub(crate) fn get_immediate_caller_address() -> Result<Key, Cep18Error> {
    let call_stack = runtime::get_call_stack();
    call_stack
        .into_iter()
        .rev()
        .nth(1)
        .map(call_stack_element_to_address)
        .ok_or(Cep18Error::InvalidContext)
}

pub fn get_total_supply_uref() -> URef {
    get_uref(TOTAL_SUPPLY)
}

pub(crate) fn read_total_supply_from(uref: URef) -> U256 {
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Writes a total supply to a specific [`URef`].
pub(crate) fn write_total_supply_to(uref: URef, value: U256) {
    storage::write(uref, value);
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SecurityBadge {
    Admin = 0,
    Minter = 1,
    None = 2,
}

impl CLTyped for SecurityBadge {
    fn cl_type() -> casper_types::CLType {
        casper_types::CLType::U8
    }
}

impl ToBytes for SecurityBadge {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        Ok(vec![*self as u8])
    }

    fn serialized_length(&self) -> usize {
        1
    }
}

impl FromBytes for SecurityBadge {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        Ok((
            match bytes[0] {
                0 => SecurityBadge::Admin,
                1 => SecurityBadge::Minter,
                2 => SecurityBadge::None,
                _ => return Err(bytesrepr::Error::LeftOverBytes),
            },
            &[],
        ))
    }
}
