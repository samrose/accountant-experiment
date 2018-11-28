#[macro_use]
extern crate hdk;
pub extern crate serde;
extern crate serde_derive;
extern crate serde_json;

pub extern crate holochain_core_types;
pub extern crate holochain_dna;
pub extern crate holochain_wasm_utils;

// see https://holochain.github.io/rust-api/0.0.1/hdk/ for info on using the hdk library

define_zome! {
    entries: [
    ]

    genesis: || {
        Ok(())
    }

    functions: {
    }
}
