//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use core::net::MSG;
use std::net::TcpStream;

// The maximum number of bytes in a protobuf varint32
const MAX_VARINT_WIDTH: u32 = 5;

pub struct Connection {
	pub stream: TcpStream,
	pub cvid: Option<i32>,
	pub uuid: Option<String>,
}

fn new(stream: TcpStream) -> Connection {
	return Connection {
		stream: stream,
		cvid: None(),
		uuid: None(),
	};
}

fn write_varint32(buffer: &[u8], value: i32) {
	for i in 0..(MAX_VARINT_WIDTH - 1) {
		if (value & !0x7f) == 0 {
			buffer[i] = value;
			break;
		} else {
			buffer[i] = ((value & 0x7f) | 0x80) & 0xff;
			value = (value as u32) >> 7;
		}
	}
}

fn read_varint32(buffer: &[u8]) -> Result<i32> {
	let mut tmp: i8 = buffer[0];
	if tmp >= 0 {
		return tmp;
	} else {
		let mut result: i32 = tmp & 127;
		if (tmp = buffer[1]) >= 0 {
			result |= tmp << 7;
		} else {
			result |= (tmp & 127) << 7;
			if (tmp = buffer[2]) >= 0 {
				result |= tmp << 14;
			} else {
				result |= (tmp & 127) << 14;
				if (tmp = buffer[3]) >= 0 {
					result |= tmp << 21;
				} else {
					result |= (tmp & 127) << 21;
					result |= (tmp = buffer[4]) << 28;
					if tmp < 0 {
						return Err();
					}
				}
			}
		}
		return Ok(result);
	}
}

fn connection_send(connection: &Connection, message: &mut MSG) -> Result {
	connection
		.stream
		.write_all(message.write_to_bytes().unwrap());
}

fn connection_recv(connection: &Connection) -> Option<MSG> {}
