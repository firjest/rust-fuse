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

use std::io;
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

	pub fn interrupted(&self) -> bool {
		todo!()
	}
}

mod private {
	pub trait Sealed {}
}

pub trait RespondOnce<Response>: private::Sealed + Send {
	fn ok(self, response: &Response);
	fn err(self, err: io::Error);

	fn into_box(self) -> Box<dyn RespondOnceBox<Response> + 'static>;
}

pub trait RespondOnceBox<Response>: private::Sealed + Send {
	fn ok(self: Box<Self>, response: &Response);
	fn err(self: Box<Self>, err: io::Error);
}

// RespondOnceImpl {{{

pub(crate) struct RespondOnceImpl<'a> {
	channel: &'a Arc<fuse_io::FileChannel>,
	request_id: u64,
	fuse_version: crate::ProtocolVersion,
}

impl<'a> RespondOnceImpl<'a> {
	pub(crate) fn new(
		channel: &'a Arc<fuse_io::FileChannel>,
		request_id: u64,
		fuse_version: crate::ProtocolVersion,
	) -> Self {
		Self {
			channel,
			request_id,
			fuse_version,
		}
	}

	fn encoder(&self) -> fuse_io::ResponseEncoder<fuse_io::FileChannel> {
		fuse_io::ResponseEncoder::new(
			self.channel,
			self.request_id,
			self.fuse_version,
		)
	}

	pub(crate) fn err_impl(self, err: io::Error) {
		self.encoder().encode_error(-err_code(err));
	}
}

impl private::Sealed for RespondOnceImpl<'_> {}

impl<Response> RespondOnce<Response> for RespondOnceImpl<'_>
where
	Response: fuse_io::EncodeResponse,
{
	fn ok(self, response: &Response) {
		response.encode_response(self.encoder());
	}

	fn err(self, err: io::Error) {
		self.err_impl(err);
	}

	fn into_box(self) -> Box<dyn RespondOnceBox<Response> + 'static> {
		Box::new(RespondOnceBoxImpl {
			channel: self.channel.clone(),
			request_id: self.request_id,
			fuse_version: self.fuse_version,
		})
	}
}

// }}}

// RespondOnceBoxImpl {{{

struct RespondOnceBoxImpl {
	channel: Arc<fuse_io::FileChannel>,
	request_id: u64,
	fuse_version: crate::ProtocolVersion,
}

impl RespondOnceBoxImpl {
	fn encoder(&self) -> fuse_io::ResponseEncoder<fuse_io::FileChannel> {
		fuse_io::ResponseEncoder::new(
			self.channel.as_ref(),
			self.request_id,
			self.fuse_version,
		)
	}
}

impl private::Sealed for RespondOnceBoxImpl {}

impl<Response> RespondOnceBox<Response> for RespondOnceBoxImpl
where
	Response: fuse_io::EncodeResponse,
{
	fn ok(self: Box<Self>, response: &Response) {
		response.encode_response(self.encoder());
	}

	fn err(self: Box<Self>, err: io::Error) {
		self.encoder().encode_error(-err_code(err));
	}
}

// }}}

fn err_code(err: io::Error) -> i32 {
	match err.raw_os_error() {
		Some(err_code) => err_code,
		None => match err.kind() {
			_ => libc::EIO,
		},
	}
}
