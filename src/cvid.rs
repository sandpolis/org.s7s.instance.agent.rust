//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use core::net::msg::RQ_Cvid;
use core::net::msg::RS_Cvid;
use core::net::MSG;
use log::{debug, info};
use std::net::TcpStream;

fn cvid_handshake(connection: &Connection) {
	let mut rq_cvid = RQ_Cvid::new();
	rq_cvid.instance = core::instance::InstanceType::AGENT;
	rq_cvid.instance_flavor = core::instance::InstanceFlavor::MICRO;
	rq_cvid.uuid = "".to_string();

	if (!Send(rq)) {
		return false;
	}

	if (!Recv(rs)) {
		return false;
	}

	debug!("Completed CVID handshake in {} ms");
	debug!(
		"Server UUID: {}, server CVID: {}",
		rs_cvid.server_uuid, rs_cvid.server_cvid
	);
	debug!("Assigned CVID: {}", rs_cvid.cvid);

	return true;
}
