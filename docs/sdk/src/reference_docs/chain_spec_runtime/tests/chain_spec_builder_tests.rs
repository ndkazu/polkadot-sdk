#![feature(proc_macro_hygiene)]
extern crate command_macros;
use command_macros::command;
use serde_json::{json, Value};
use std::{process::Command, str};

const WASM_FILE_PATH: &str =
	"../../../../../target/release/wbuild/chain-spec-guide-runtime/chain_spec_guide_runtime.wasm";

const CHAIN_SPEC_BUILDER_PATH: &str = "../../../../../target/release/chain-spec-builder";

fn get_chain_spec_builder_path() -> &'static str {
	// dev-dependencies do not build binary. So let's do the naive work-around here:
	let _ = command!(
		cargo build --release -p staging-chain-spec-builder --bin chain-spec-builder
	).status().expect("Failed to execute command");
	CHAIN_SPEC_BUILDER_PATH
}

#[test]
#[docify::export]
fn list_presets() {
	let path = get_chain_spec_builder_path();
	let output = command!(
		(path) list_presets -r (WASM_FILE_PATH)
	).output().expect("Failed to execute command");

	let output: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();

	let expected_output = json!({
		"presets":[
			"preset_1",
			"preset_2",
			"preset_3",
			"preset_4",
			"preset_invalid"
		]
	});
	assert_eq!(output, expected_output, "Output did not match expected");
}

#[test]
#[docify::export]
fn get_preset() {
	let path = get_chain_spec_builder_path();
	let output = command!(
		(path) display-preset -r (WASM_FILE_PATH) -p preset-2
	).output().expect("Failed to execute command");
	let output: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();

	//note: copy of chain_spec_guide_runtime::preset_2
	let expected_output = json!({
		"bar": {
			"initialAccount": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
		},
		"foo": {
			"someEnum": {
				"Data2": {
					"values": "0x0c10"
				}
			},
			"someInteger": 200
		},
	});
	assert_eq!(output, expected_output, "Output did not match expected");
}

#[test]
#[docify::export]
fn generate_chain_spec() {
	let path = get_chain_spec_builder_path();
	let output = command!(
		(path) -c /dev/stdout create -r (WASM_FILE_PATH) named-preset preset_2
	).output().expect("Failed to execute command");
	let mut output: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();

	//remove code field for better readability
	if let Some(code) = output["genesis"]["runtimeGenesis"].as_object_mut().unwrap().get_mut("code")
	{
		*code = Value::String("0x123".to_string());
	}

	let expected_output = json!({
	  "name": "Custom",
	  "id": "custom",
	  "chainType": "Live",
	  "bootNodes": [],
	  "telemetryEndpoints": null,
	  "protocolId": null,
	  "properties": { "tokenDecimals": 12, "tokenSymbol": "UNIT" },
	  "codeSubstitutes": {},
	  "genesis": {
		"runtimeGenesis": {
		  "code": "0x123",
		  "patch": {
			"bar": {
			  "initialAccount": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL"
			},
			"foo": {
			  "someEnum": {
				"Data2": {
				  "values": "0x0c10"
				}
			  },
			  "someInteger": 200
			}
		  }
		}
	  }
	});
	assert_eq!(output, expected_output, "Output did not match expected");
}

#[test]
#[docify::export]
fn generate_para_chain_spec() {
	let path = get_chain_spec_builder_path();
	let output = command!(
		(path) -c /dev/stdout create -c polkadot -p 1000 -r (WASM_FILE_PATH) named-preset preset2
		).output().expect("Failed to execute command");

	let mut output: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();

	//remove code field for better readability
	if let Some(code) = output["genesis"]["runtimeGenesis"].as_object_mut().unwrap().get_mut("code")
	{
		*code = Value::String("0x123".to_string());
	}

	let expected_output = json!({
	  "name": "Custom",
	  "id": "custom",
	  "chainType": "Live",
	  "bootNodes": [],
	  "telemetryEndpoints": null,
	  "protocolId": null,
	  "relay_chain": "polkadot",
	  "para_id": 1000,
	  "properties": { "tokenDecimals": 12, "tokenSymbol": "UNIT" },
	  "codeSubstitutes": {},
	  "genesis": {
		"runtimeGenesis": {
		  "code": "0x123",
		  "patch": {
			"bar": {
			  "initialAccount": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL"
			},
			"foo": {
			  "someEnum": {
				"Data2": {
				  "values": "0x0c10"
				}
			  },
			  "someInteger": 200
			}
		  }
		}
	  }
	});
	assert_eq!(output, expected_output, "Output did not match expected");
}

#[test]
#[docify::export]
fn preset_4_json() {
	assert_eq!(
		chain_spec_guide_runtime::presets::preset_4(),
		json!({
			"foo": {
				"someEnum": {
					"Data2": {
						"values": "0x0c10"
					}
				},
			},
		})
	);
}
