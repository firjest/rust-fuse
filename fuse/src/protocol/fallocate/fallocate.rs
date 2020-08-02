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

use crate::protocol::prelude::*;

#[cfg(test)]
mod fallocate_test;

// FallocateRequest {{{

pub struct FallocateRequest<'a> {
	header: &'a fuse_kernel::fuse_in_header,
	raw: &'a fuse_kernel::fuse_fallocate_in,
}

impl FallocateRequest<'_> {
	pub fn node_id(&self) -> u64 {
		self.header.nodeid
	}

	pub fn handle(&self) -> u64 {
		self.raw.fh
	}

	pub fn offset(&self) -> u64 {
		self.raw.offset
	}

	pub fn length(&self) -> u64 {
		self.raw.length
	}

	pub fn mode(&self) -> u32 {
		self.raw.mode
	}
}

impl<'a> fuse_io::DecodeRequest<'a> for FallocateRequest<'a> {
	fn decode_request(
		mut dec: fuse_io::RequestDecoder<'a>,
	) -> io::Result<Self> {
		let header = dec.header();
		debug_assert!(header.opcode == fuse_kernel::FUSE_FALLOCATE);
		let raw = dec.next_sized()?;
		Ok(Self { header, raw })
	}
}

// }}}

// FallocateResponse {{{

pub struct FallocateResponse<'a> {
	phantom: PhantomData<&'a ()>,
}

impl FallocateResponse<'_> {
	pub fn new() -> Self {
		FallocateResponse {
			phantom: PhantomData,
		}
	}
}

impl fmt::Debug for FallocateResponse<'_> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.debug_struct("FallocateResponse").finish()
	}
}

impl fuse_io::EncodeResponse for FallocateResponse<'_> {
	fn encode_response<'a, Chan: fuse_io::Channel>(
		&'a self,
		enc: fuse_io::ResponseEncoder<Chan>,
	) -> std::io::Result<()> {
		enc.encode_header_only()
	}
}

// }}}