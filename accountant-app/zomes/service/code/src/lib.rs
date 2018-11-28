#[macro_use]
extern crate hdk;
pub extern crate serde;
extern crate serde_derive;
extern crate serde_json;

// see https://holochain.github.io/rust-api/0.0.1/hdk/ for info on using the hdk library

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct ServiceCycle {
    pub agent_key: String,
    pub request_hash: HashString,
    pub response_hash: HashString,
    pub metrics: ServiceMetrics,
    pub signature: Option<String>,
}

fn handle_log(log: ServiceCycle) -> serde_json::Value {
	match hdk::commit_entry("log", json!(ServiceCycle)) {}
}
define_zome! {
    entries: [
        entry!(
            name: "service-logger",
            description: "A log of a single request_payload/response_payload cycle",
            sharing: Sharing::Public,
            native_type: ServiceCycle,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |service_cycle: ServiceCycle, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    ]

    genesis: || {
        Ok(())
    }

    functions: {
    }
}
