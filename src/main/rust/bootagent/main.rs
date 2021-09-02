//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

pub mod snapshot {
	pub mod ui;
	pub mod utils;
}

#[path = "../gen"]
pub mod core {

	#[path = "com.sandpolis.core.foundation"]
	pub mod foundation {
		pub mod platform;
		pub mod result;
	}

	#[path = "com.sandpolis.core.instance"]
	pub mod instance {
		pub mod auth;
		pub mod group;
		pub mod metatypes;
	}

	#[path = "com.sandpolis.core.net"]
	pub mod net {
		pub mod message;
		pub mod msg_cvid;
	}
}

#[path = "../gen"]
pub mod plugin {

	#[path = "com.sandpolis.plugin.snapshot"]
	pub mod snapshot {
		pub mod msg_snapshot;
	}
}

#[path = "../lib"]
pub mod lib {
	pub mod connection;
	pub mod messages;
	pub mod uuid;
}

use rust_embed::RustEmbed;

/// Contains embedded resources
#[derive(RustEmbed)]
#[folder = "src/main/resources/bootagent"]
struct BinaryAssets;

fn main() {
	env_logger::init();
	crate::snapshot::ui::start();
}
