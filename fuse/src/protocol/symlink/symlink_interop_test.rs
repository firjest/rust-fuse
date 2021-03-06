// Copyright 2021 John Millikin and the rust-fuse contributors.
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

use std::sync::mpsc;
use std::{ffi, panic};

use interop_testutil::{diff_str, errno, interop_test, path_cstr};

struct TestFS {
	requests: mpsc::Sender<String>,
}

impl fuse::FuseHandlers for TestFS {
	fn lookup(
		&self,
		_ctx: fuse::ServerContext,
		request: &fuse::LookupRequest,
		respond: impl for<'a> fuse::Respond<fuse::LookupResponse<'a>>,
	) {
		if request.parent_id() != fuse::ROOT_ID {
			respond.err(fuse::ErrorCode::ENOENT);
			return;
		}
		if request.name() != fuse::NodeName::from_bytes(b"exists.txt").unwrap()
		{
			respond.err(fuse::ErrorCode::ENOENT);
			return;
		}

		let mut resp = fuse::LookupResponse::new();
		let node = resp.node_mut();
		node.set_id(fuse::NodeId::new(2).unwrap());
		node.set_cache_timeout(std::time::Duration::from_secs(60));

		let attr = node.attr_mut();
		attr.set_mode(fuse::FileType::Regular | 0o644);
		attr.set_nlink(1);

		respond.ok(&resp);
	}

	fn symlink(
		&self,
		_ctx: fuse::ServerContext,
		request: &fuse::SymlinkRequest,
		respond: impl for<'a> fuse::Respond<fuse::SymlinkResponse<'a>>,
	) {
		self.requests.send(format!("{:#?}", request)).unwrap();

		let mut resp = fuse::SymlinkResponse::new();
		let node = resp.node_mut();
		node.set_id(fuse::NodeId::new(3).unwrap());

		let attr = node.attr_mut();
		attr.set_mode(fuse::FileType::Symlink | 0o644);
		attr.set_nlink(1);

		respond.ok(&resp);
	}
}

fn symlink_test(
	test_fn: impl FnOnce(&std::path::Path) + panic::UnwindSafe,
) -> Vec<String> {
	let (request_send, request_recv) = mpsc::channel();
	let fs = TestFS {
		requests: request_send,
	};
	interop_test(fs, test_fn);
	request_recv.iter().collect()
}

#[test]
fn symlink() {
	let requests = symlink_test(|root| {
		let path = path_cstr(root.join("symlink.txt"));
		let target = ffi::CString::new("symlink_target.txt").unwrap();

		let rc = unsafe { libc::symlink(target.as_ptr(), path.as_ptr()) };
		assert_eq!(rc, 0);
	});
	assert_eq!(requests.len(), 1);

	let expect = r#"SymlinkRequest {
    parent_id: 1,
    name: "symlink_target.txt",
    content: "symlink.txt",
}"#;
	if let Some(diff) = diff_str(expect, &requests[0]) {
		println!("{}", diff);
		assert!(false);
	}
}

#[test]
fn symlink_err_eexist() {
	let requests = symlink_test(|root| {
		let path = path_cstr(root.join("exists.txt"));
		let target = ffi::CString::new("symlink_target.txt").unwrap();

		let rc = unsafe { libc::symlink(target.as_ptr(), path.as_ptr()) };
		assert_eq!(rc, -1);
		assert_eq!(errno(), libc::EEXIST);
	});
	assert_eq!(requests.len(), 0);
}
