//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use dotproperties::parse_from_slice;
use log::{debug, info};
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::net::TcpStream;
use std::{thread, time};

#[path = "../../module"]
pub mod core {

	#[path = "com.sandpolis.core.foundation/gen/main/rust"]
	pub mod foundation {
		#[path = "platform.rs"] pub mod platform;
		#[path = "result.rs"] pub mod result;
	}

	#[path = "com.sandpolis.core.instance/gen/main/rust"]
	pub mod instance {
		#[path = "auth.rs"] pub mod auth;
		#[path = "group.rs"] pub mod group;
	}

	#[path = "com.sandpolis.core.net/gen/main/rust"]
	pub mod net {
		#[path = "message.rs"] pub mod message;
	}
}

pub mod connection;
pub mod uuid;

use protobuf::Message;
use crate::core::instance::group::*;

#[derive(RustEmbed)]
#[folder = "res"]
struct Assets;

fn main() {
	// Load build metadata
	if let Some(build_properties) = Assets::get("build.properties") {
		if let Ok(properties_vector) = parse_from_slice(&build_properties) {
			let properties: HashMap<_, _> = properties_vector.into_iter().collect();

			debug!("Build platform: {}", properties["build.platform"]);
			debug!("Build JVM version: {}", properties["build.java_version"]);
		}
	} else {
		println!("Failed to locate embedded build.properties resource")
	}

	// Load agent configuration
	if let Some(config_bytes) = Assets::get("config.bin") {
		if let Ok(config) = AgentConfig::parse_from_bytes(&config_bytes) {}
	} else {
		println!("Failed to locate embedded configuration")
	}
}

fn connection_routine(config: &AgentConfig_LoopConfig) {
	let mut iteration: i32 = 0;
	while iteration < config.iteration_limit || config.iteration_limit == 0 {
		iteration += 1;
		if let Ok(stream) = TcpStream::connect("127.0.0.1:8768") {

			// Perform CVID handshake
		}

		thread::sleep(time::Duration::from_millis(config.cooldown as u64));
	}
}
