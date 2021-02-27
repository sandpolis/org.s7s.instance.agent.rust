//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use crate::core::net::msg::*;
use crate::core::instance::metatypes::*;
use core::net::MSG;
use log::{debug, info};
use std::net::TcpStream;
use std::time::{Duration, Instant};

fn cvid_handshake(connection: &Connection, uuid: &String) -> Result<(), CvidHandshakeError> {

	let operation_start = Instant::now();

	let mut rq_cvid = RQ_Cvid::new();
	rq_cvid.instance = InstanceType::AGENT;
	rq_cvid.instance_flavor = InstanceFlavor::MICRO;
	rq_cvid.uuid = uuid;

	let rq = new_rq(rq_cvid);

	if let Err(error) = connection.send(&rq) {
		// TODO
	}

	if let Ok(rs) = connection.recv() {
		if let Ok(rs_cvid) = RS_Cvid::parse_from_bytes(&rs.payload) {
			debug!("Completed CVID handshake in {:?} ms", operation_start.elapsed());
			debug!("Discovered server UUID: {}", rs_cvid.server_uuid);
			debug!("Discovered server CVID: {}", rs_cvid.server_cvid);
		} else {
			return Err()
		}
	} else {
		return Err()
	}

	return Ok()
}
