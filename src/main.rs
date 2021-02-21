//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use std::net::TcpStream;

use rust_embed::RustEmbed;
use core::instance::AgentConfig

#[derive(RustEmbed)]
#[folder = "src/resources"]
struct Assets;

fn main() {

	// Load build metadata
	if let Some(build_properties) = Assets::get("build.properties") {

	} else {
		println!("Failed to locate embedded build.properties resource")
	}

	// Load agent configuration
	if let Some(config_bytes) = Assets::get("config.bin") {
		if let Some(config) = AgentConfig::parse_from_bytes(&config_bytes) {

		}
	} else {
		println!("Failed to locate embedded configuration")
	}
}

/*const core::instance::LoopConfig &loop_config =
	config.network().loop_config();
	while (iteration < loop_config.iteration_limit()
	|| loop_config.iteration_limit() == 0) {

	for (int i = 0; i < loop_config.target_size(); ++i) {
	std::cout << "Attempting connection: "
	<< loop_config.target(i).address() << std::endl;
	int sfd = OpenConnection(loop_config.target(i).address(),
	loop_config.target(i).port());
	if (sfd > 0) {
	Sock sock(uuid, sfd);

	if (sock.CvidHandshake()) {
	// TODO enter sock event loop
	return 0;
	}

	iteration = 0;
	break;
	}

	iteration++;
	std::this_thread::sleep_for(
	std::chrono::milliseconds(loop_config.cooldown()));
	}
	}*/

fn connection_routine() {
	while (iteration < agent_config.loop_config.iteration_limit || agent_config.loop_config.iteration_limit == 0) {
		if let Ok(stream) = TcpStream::connect("127.0.0.1:8768") {

		}
	}
}