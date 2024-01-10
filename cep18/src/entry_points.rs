//! Contains definition of the entry points.
use alloc::{string::String, vec::Vec};

use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints};

use crate::constants::{INIT_ENTRY_POINT_NAME, INSCRIBE_ENTRY_POINT_NAME};

/// Returns the `init` entry point.
pub fn init() -> EntryPoint {
    EntryPoint::new(
        String::from(INIT_ENTRY_POINT_NAME),
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `init` entry point.
pub fn inscribe() -> EntryPoint {
    EntryPoint::new(
        String::from(INSCRIBE_ENTRY_POINT_NAME),
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of CEP-18 token entry points.
pub fn generate_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(init());
    entry_points.add_entry_point(inscribe());
    entry_points
}
