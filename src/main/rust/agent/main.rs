//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

#[path = "../../../../../module"]
pub mod core {

	#[path = "com.sandpolis.core.foundation/gen/main/rust"]
	pub mod foundation {
		pub mod platform;
		pub mod result;
	}

	#[path = "com.sandpolis.core.instance/gen/main/rust"]
	pub mod instance {
		pub mod auth;
		pub mod group;
		pub mod metatypes;
	}

	#[path = "com.sandpolis.core.net/gen/main/rust"]
	pub mod net {
		pub mod message;
		pub mod msg_cvid;
	}
}

#[path = "../lib"]
pub mod lib {
	pub mod connection;
	pub mod messages;
	pub mod uuid;
}

use crate::core::instance::group::*;
use dotproperties::parse_from_slice;
use log::{debug, info, error};
use protobuf::Message;
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::net::TcpStream;
use std::{thread, time};

/// Contains embedded resources
#[derive(RustEmbed)]
#[folder = "res"]
struct BinaryAssets;

fn main() -> Result<(), Error> {

	// Initialize logging
	env_logger::init();

	// Load build metadata
	if let Some(build_properties) = BinaryAssets::get("build.properties") {
		let properties = parse_from_slice(&build_properties)?.into_iter().collect();

		// Output debug build info
		debug!("Build platform: {}", properties["build.platform"]);
		debug!("Build JVM version: {}", properties["build.java_version"]);

		info!("Starting Sandpolis agent v{}", properties["build.version"]);
	} else {
		error!("Failed to locate embedded build.properties resource")
		return Err(Error)
	}

	// Load agent configuration
	if let Some(config_bytes) = BinaryAssets::get("config.bin") {
		if let Ok(config) = AgentConfig::parse_from_bytes(&config_bytes) {

		} else {
			error!("The embedded configuration is invalid")
		}
	} else {
		debug!("Failed to locate embedded configuration")
	}

	// Prompt user
	info!("Preparing to configure agent");
	print!("Please enter the server's address [127.0.0.1]: ");

	let mut server_host = String::new();
	std::io::stdin().read_line(&mut server_host)?;

	if server_host == "" {
		server_host = "127.0.0.1".to_string();
	}

	// Attempt a connection
	info!("Attempting connection to server");
	if let Ok(connection) = Connection::new(server_host, 8768) {
		// TODO
	}

	println!("Choose authentication scheme:");
	println!("  1. Password");
	println!("  2. None");

	return Ok(())
}

fn connection_routine(config: &AgentConfig_LoopConfig) {

	debug!("Starting connection routine");

	let mut iteration: i32 = 0;
	while iteration < config.iteration_limit || config.iteration_limit == 0 {
		iteration += 1;
		if let Ok(connection) = Connection::new("127.0.0.1", 8768) {

		}

		thread::sleep(time::Duration::from_millis(config.cooldown as u64));
	}
}

fn dispatch_routine(connection: &mut Connection) {

	debug!("Starting command dispatch routine");

	loop {
		// TODO
	}
}
