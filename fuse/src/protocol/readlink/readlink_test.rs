// Copyright 2020 John Millikin and the rust-fuse contributors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use crate::internal::testutil::MessageBuilder;
use crate::protocol::prelude::*;

use super::{ReadlinkRequest, ReadlinkResponse};

#[test]
fn request_empty() {
	let buf = MessageBuilder::new()
		.set_opcode(fuse_kernel::FUSE_READLINK)
		.build_aligned();

	let _req: ReadlinkRequest = decode_request!(buf);
}

#[test]
fn response() {
	let mut resp = ReadlinkResponse::new();
	resp.set_name(CStr::from_bytes_with_nul(b"hello.world!\x00").unwrap());

	let encoded = encode_response!(resp);

	assert_eq!(
		encoded,
		MessageBuilder::new()
			.push_sized(&fuse_kernel::fuse_out_header {
				len: (size_of::<fuse_kernel::fuse_out_header>() + 13) as u32,
				error: 0,
				unique: 0,
			})
			.push_bytes(b"hello.world!\x00")
			.build()
	);
}