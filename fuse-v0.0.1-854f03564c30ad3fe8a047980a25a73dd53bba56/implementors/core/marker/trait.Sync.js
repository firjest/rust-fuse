(function() {var implementors = {};
implementors["fuse"] = [{"text":"impl Sync for CuseDeviceName","synthetic":true,"types":[]},{"text":"impl&lt;Channel, Handlers, Hooks&gt; Sync for CuseServer&lt;Channel, Handlers, Hooks&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Channel: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Handlers: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Hooks: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, Channel, Handlers, Hooks&gt; Sync for CuseServerBuilder&lt;'a, Channel, Handlers, Hooks&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Channel: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Handlers: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Hooks: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Channel, Handlers, Hooks&gt; Sync for CuseServerExecutor&lt;Channel, Handlers, Hooks&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Channel: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Handlers: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Hooks: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Channel, Handlers, Hooks&gt; Sync for FuseServer&lt;Channel, Handlers, Hooks&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Channel: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Handlers: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Hooks: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Channel, Handlers, Hooks&gt; Sync for FuseServerBuilder&lt;Channel, Handlers, Hooks&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Channel: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Handlers: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Hooks: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Channel, Handlers, Hooks&gt; Sync for FuseServerExecutor&lt;Channel, Handlers, Hooks&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Channel: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Handlers: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Hooks: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for ServerContext","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; Sync for RespondAsync&lt;R&gt;","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]},{"text":"impl Sync for ErrorCode","synthetic":true,"types":[]},{"text":"impl Sync for FileMode","synthetic":true,"types":[]},{"text":"impl Sync for Node","synthetic":true,"types":[]},{"text":"impl Sync for NodeAttr","synthetic":true,"types":[]},{"text":"impl Sync for NodeId","synthetic":true,"types":[]},{"text":"impl Sync for NodeName","synthetic":true,"types":[]},{"text":"impl Sync for XattrName","synthetic":true,"types":[]},{"text":"impl Sync for FileType","synthetic":true,"types":[]},{"text":"impl Sync for ProtocolVersion","synthetic":true,"types":[]},{"text":"impl Sync for OpcodeEnum","synthetic":true,"types":[]},{"text":"impl&lt;Handlers, Hooks&gt; Sync for CuseServerBuilder&lt;Handlers, Hooks&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Handlers: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Hooks: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for DevCuseChannel","synthetic":true,"types":[]},{"text":"impl Sync for DevFuseChannel","synthetic":true,"types":[]},{"text":"impl&lt;Mount, Handlers, Hooks&gt; Sync for FuseServerBuilder&lt;Mount, Handlers, Hooks&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Handlers: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Hooks: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;Mount: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for LibcFuseMount","synthetic":true,"types":[]},{"text":"impl Sync for SyscallFuseMount","synthetic":true,"types":[]},{"text":"impl Sync for RequestHeader","synthetic":true,"types":[]},{"text":"impl Sync for ResponseHeader","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !Sync for UnknownRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for XattrError","synthetic":true,"types":[]},{"text":"impl Sync for AccessRequest","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for AccessResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for BmapRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for BmapResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for CreateRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for CreateResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for CuseInitRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for CuseInitResponse","synthetic":true,"types":[]},{"text":"impl Sync for CuseInitFlags","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for FallocateRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for FallocateResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for FlushRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for FlushResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for ForgetRequestItem","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ForgetRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for FsyncRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for FsyncResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for FsyncdirRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for FsyncdirResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for FuseInitRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for FuseInitResponse","synthetic":true,"types":[]},{"text":"impl Sync for FuseInitFlags","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for GetattrRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for GetattrResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for GetlkRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for GetlkResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for GetxattrRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for GetxattrResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for IoctlRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for IoctlResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for LinkRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for LinkResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ListxattrRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ListxattrResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for LookupRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for LookupResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for LseekRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for LseekResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for MkdirRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for MkdirResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for MknodRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for MknodResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for OpenRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for OpenResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for OpenResponseFlags","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for OpendirRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for OpendirResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for OpendirResponseFlags","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ReadRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ReadResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ReaddirRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for ReaddirError","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ReaddirResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for ReaddirEntry","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ReadlinkRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ReadlinkResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ReleaseRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ReleaseResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ReleasedirRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ReleasedirResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for RemovexattrRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for RemovexattrResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for RenameRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for RenameRequestFlags","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for RenameResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for RmdirRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for RmdirResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for SetattrRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for SetattrResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for SetlkRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for SetlkResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for SetxattrRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for SetxattrResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for StatfsRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for StatfsResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for SymlinkRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for SymlinkResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for UnlinkRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for UnlinkResponse&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for WriteRequest&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for WriteRequestFlags","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for WriteResponse&lt;'a&gt;","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()