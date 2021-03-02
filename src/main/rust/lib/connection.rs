//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use crate::core::instance::metatypes::*;
use crate::core::net::message::MSG;
use crate::core::net::msg_cvid::*;
use crate::lib::messages::rq;
use log::{debug, info};
use protobuf::Message;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct CvidHandshakeError;
pub enum MessageSendError {
	ConnectionClosed,
	Other,
}

pub enum MessageRecvError {
	ConnectionClosed,
	Other,
}

pub enum ConnectionState {
	NotConnected,
}

pub struct Connection {

	/// The underlying transport stream
	pub stream: TcpStream,

	/// The connection state
	pub state: ConnectionState,

	/// The remote CVID
	pub cvid: Option<i32>,

	/// The remote UUID
	pub uuid: Option<String>,

	receive_map: Mutex<HashMap<i32, MSG>>,
}

impl Connection {

	pub fn send(&mut self, message: &MSG) -> Result<(), MessageSendError> {
		self.stream.write_all(&message.write_to_bytes().unwrap());
		return Ok(())
	}

	pub fn recv(&self, id: i32) -> Result<MSG, MessageRecvError> {

		// First check the receive map
		let mut receive_map = self.receive_map.lock().unwrap();
		if let Some(msg) = receive_map.remove(&id) {
			return Ok(msg);
		}

		// TODO read stream
		return Err(MessageRecvError::Other);
	}

	fn cvid_handshake(&mut self, uuid: String) -> Result<i32, CvidHandshakeError> {

		let operation_start = Instant::now();

		let mut rq_cvid = RQ_Cvid::new();
		rq_cvid.instance = InstanceType::AGENT;
		rq_cvid.instance_flavor = InstanceFlavor::MICRO;
		rq_cvid.uuid = uuid;

		let rq = rq(&rq_cvid);

		if let Err(error) = self.send(&rq) {
			// TODO
		}

		if let Ok(rs) = self.recv(rq.id) {
			if let Ok(rs_cvid) = RS_Cvid::parse_from_bytes(&rs.payload) {
				debug!("Completed CVID handshake in {:?} ms", operation_start.elapsed());
				debug!("Assigned CVID: {}", rs_cvid.cvid);
				debug!("Discovered server UUID: {}", rs_cvid.server_uuid);
				debug!("Discovered server CVID: {}", rs_cvid.server_cvid);
				return Ok(rs_cvid.cvid);
			}
		}

		return Err(CvidHandshakeError);
	}
}

fn new(stream: TcpStream) -> Connection {
	return Connection {
		stream: stream,
		state: ConnectionState::NotConnected,
		cvid: None,
		uuid: None,
		receive_map: Mutex::new(HashMap::new()),
	};
}
