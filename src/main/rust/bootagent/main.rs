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

#[path = "../../../../../module"]
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
		#[path = "metatypes.rs"] pub mod metatypes;
	}

	#[path = "com.sandpolis.core.net/gen/main/rust"]
	pub mod net {
		#[path = "message.rs"] pub mod message;
		#[path = "msg_cvid.rs"] pub mod msg_cvid;
	}
}

#[path = "../lib"]
pub mod lib {
	pub mod connection;
	pub mod messages;
	pub mod uuid;
}

fn main() {
	env_logger::init();
	crate::snapshot::ui::start();
}
