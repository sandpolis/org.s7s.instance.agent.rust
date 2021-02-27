//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use crate::core::net::message::MSG;
use protobuf::Message;
use std::net::TcpStream;

/// The maximum number of bytes in a protobuf varint32
const MAX_VARINT_WIDTH: u32 = 5;

pub struct Connection {

	/// The underlying transport stream
	pub stream: TcpStream,

	/// The remote CVID
	pub cvid: Option<i32>,

	/// The remote UUID
	pub uuid: Option<String>,
}

impl Connection {

	fn send(&self, message: &MSG) -> Result<(), SendError> {
		connection
			.stream
			.write_all(message.write_to_bytes().unwrap());
	}

	fn recv(&self) -> Option<MSG> {}
}

fn new(stream: TcpStream) -> Connection {
	return Connection {
		stream: stream,
		cvid: None(),
		uuid: None(),
	};
}
