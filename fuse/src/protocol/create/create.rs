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

use crate::protocol::common;
use crate::protocol::node;
use crate::protocol::prelude::*;

#[cfg(test)]
mod create_test;

// CreateRequest {{{

/// **\[UNSTABLE\]** Request type for [`FuseHandlers::create`].
///
/// [`FuseHandlers::create`]: ../trait.FuseHandlers.html#method.create
#[derive(Debug)]
pub struct CreateRequest<'a> {
	node_id: node::NodeId,
	name: &'a CStr,
	flags: u32,
	mode: u32,
	umask: u32,
}

impl CreateRequest<'_> {
	pub fn node_id(&self) -> node::NodeId {
		self.node_id
	}

	pub fn name(&self) -> &CStr {
		self.name
	}

	pub fn flags(&self) -> u32 {
		self.flags
	}

	pub fn mode(&self) -> u32 {
		self.mode
	}

	pub fn umask(&self) -> u32 {
		self.umask
	}
}

#[repr(C)]
struct fuse_create_in_v7p1 {
	pub flags: u32,
	pub unused: u32,
}

impl<'a> fuse_io::DecodeRequest<'a> for CreateRequest<'a> {
	fn decode_request(
		mut dec: fuse_io::RequestDecoder<'a>,
	) -> io::Result<Self> {
		let header = dec.header();
		debug_assert!(header.opcode == fuse_kernel::FUSE_CREATE);

		let node_id = try_node_id(header.nodeid)?;

		if dec.version().minor() < 12 {
			let raw: &'a fuse_create_in_v7p1 = dec.next_sized()?;
			let name = dec.next_cstr()?;
			return Ok(Self {
				node_id,
				name,
				flags: raw.flags,
				mode: 0,
				umask: 0,
			});
		}

		let raw: &'a fuse_kernel::fuse_create_in = dec.next_sized()?;
		let name = dec.next_cstr()?;
		Ok(Self {
			node_id,
			name,
			flags: raw.flags,
			mode: raw.mode,
			umask: raw.umask,
		})
	}
}

// }}}

// CreateResponse {{{

/// **\[UNSTABLE\]** Response type for [`FuseHandlers::create`].
///
/// [`FuseHandlers::create`]: ../trait.FuseHandlers.html#method.create
pub struct CreateResponse<'a> {
	phantom: PhantomData<&'a ()>,
	entry_out: fuse_kernel::fuse_entry_out,
	open_out: fuse_kernel::fuse_open_out,
}

impl CreateResponse<'_> {
	pub fn new() -> Self {
		CreateResponse {
			phantom: PhantomData,
			entry_out: Default::default(),
			open_out: Default::default(),
		}
	}

	/*
	if (!S_ISREG(outentry.attr.mode) || invalid_nodeid(outentry.nodeid))
		err -EIO
	*/

	pub fn node(&self) -> &node::Node {
		node::Node::new_ref(&self.entry_out)
	}

	pub fn node_mut(&mut self) -> &mut node::Node {
		node::Node::new_ref_mut(&mut self.entry_out)
	}

	pub fn set_handle(&mut self, handle: u64) {
		self.open_out.fh = handle;
	}

	pub fn flags(&self) -> u32 {
		self.open_out.open_flags
	}

	pub fn set_flags(&mut self, flags: u32) {
		self.open_out.open_flags = flags;
	}
}

impl fmt::Debug for CreateResponse<'_> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.debug_struct("CreateResponse").finish()
		// TODO
	}
}

impl fuse_io::EncodeResponse for CreateResponse<'_> {
	fn encode_response<'a, Chan: fuse_io::Channel>(
		&'a self,
		enc: fuse_io::ResponseEncoder<Chan>,
	) -> std::io::Result<()> {
		common::encode_entry_sized(enc, &self.entry_out, &self.open_out)
	}
}

// }}}
