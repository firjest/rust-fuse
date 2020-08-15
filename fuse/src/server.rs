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

#[cfg(not(feature = "no_std"))]
use std::sync::Arc;

use crate::internal::fuse_io;
use crate::internal::fuse_kernel;

pub struct ServerContext {
	header: fuse_kernel::fuse_in_header,
}

impl<'a> ServerContext {
	pub(crate) fn new(header: fuse_kernel::fuse_in_header) -> Self {
		Self { header }
	}

	pub fn request_id(&self) -> u64 {
		self.header.unique
	}

	pub fn user_id(&self) -> u32 {
		self.header.uid
	}

	pub fn group_id(&self) -> u32 {
		self.header.gid
	}

	pub fn process_id(&self) -> u32 {
		self.header.pid
	}
}

mod private {
	pub trait Sealed {}
}

/// **\[SEALED\]**
pub trait RespondOnce<Response>: private::Sealed {
	fn ok(self, response: &Response);
	fn err(self, err: crate::ErrorCode);

	#[cfg(not(feature = "no_std"))]
	#[cfg_attr(doc, doc(cfg(not(feature = "no_std"))))]
	fn into_box(self) -> Box<dyn RespondOnceBox<Response>>;
}

/// **\[SEALED\]**
#[cfg(not(feature = "no_std"))]
#[cfg_attr(doc, doc(cfg(not(feature = "no_std"))))]
pub trait RespondOnceBox<Response>: private::Sealed + Send + 'static {
	fn ok(self: Box<Self>, response: &Response);
	fn err(self: Box<Self>, err: crate::ErrorCode);
}

// RespondOnceImpl {{{

#[cfg(not(feature = "no_std"))]
pub(crate) struct RespondOnceImpl<'a, C> {
	channel: &'a Arc<C>,
	request_id: u64,
	fuse_version: crate::ProtocolVersion,
}

#[cfg(not(feature = "no_std"))]
impl<'a, C> RespondOnceImpl<'a, C>
where
	C: fuse_io::Channel,
{
	pub(crate) fn new(
		channel: &'a Arc<C>,
		request_id: u64,
		fuse_version: crate::ProtocolVersion,
	) -> Self {
		Self {
			channel,
			request_id,
			fuse_version,
		}
	}

	pub(crate) fn encoder(&self) -> fuse_io::ResponseEncoder<C> {
		fuse_io::ResponseEncoder::new(
			self.channel,
			self.request_id,
			self.fuse_version,
		)
	}
}

#[cfg(not(feature = "no_std"))]
impl<C> private::Sealed for RespondOnceImpl<'_, C> {}

#[cfg(not(feature = "no_std"))]
impl<C, Response> RespondOnce<Response> for RespondOnceImpl<'_, C>
where
	C: fuse_io::Channel + Send + Sync + 'static,
	Response: fuse_io::EncodeResponse,
{
	fn ok(self, response: &Response) {
		response.encode_response(self.encoder());
	}

	fn err(self, err: crate::ErrorCode) {
		self.encoder().encode_error(err);
	}

	fn into_box(self) -> Box<dyn RespondOnceBox<Response>> {
		Box::new(RespondOnceBoxImpl {
			channel: self.channel.clone(),
			request_id: self.request_id,
			fuse_version: self.fuse_version,
		})
	}
}

// }}}

// RespondOnceBoxImpl {{{

#[cfg(not(feature = "no_std"))]
struct RespondOnceBoxImpl<C> {
	channel: Arc<C>,
	request_id: u64,
	fuse_version: crate::ProtocolVersion,
}

#[cfg(not(feature = "no_std"))]
impl<C> RespondOnceBoxImpl<C>
where
	C: fuse_io::Channel,
{
	fn encoder(&self) -> fuse_io::ResponseEncoder<C> {
		fuse_io::ResponseEncoder::new(
			self.channel.as_ref(),
			self.request_id,
			self.fuse_version,
		)
	}
}

#[cfg(not(feature = "no_std"))]
impl<C> private::Sealed for RespondOnceBoxImpl<C> {}

#[cfg(not(feature = "no_std"))]
impl<C, Response> RespondOnceBox<Response> for RespondOnceBoxImpl<C>
where
	C: fuse_io::Channel + Send + Sync + 'static,
	Response: fuse_io::EncodeResponse,
{
	fn ok(self: Box<Self>, response: &Response) {
		response.encode_response(self.encoder());
	}

	fn err(self: Box<Self>, err: crate::ErrorCode) {
		self.encoder().encode_error(err);
	}
}

// }}}
