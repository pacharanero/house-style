# MCP implementations

Reference: [Model Context Protocol specification](https://modelcontextprotocol.io/specification/latest), [official Rust SDK](https://github.com/modelcontextprotocol/rust-sdk), and [`rmcp` issue #1030](https://github.com/modelcontextprotocol/rust-sdk/issues/1030).

Use the official SDK for a protocol rather than maintaining hand-written framing, lifecycle negotiation, message models, and error semantics in every application. For Rust MCP clients and servers, the target standard is `rmcp`.

## Adoption gate

- Adopt the latest stable `rmcp` release once that release contains the bounded stdio reader fix tracked by `modelcontextprotocol/rust-sdk#1030` / PR `#1049`, or an equivalent upstream fix with a finite default limit and a configurable override.
- Verify the release and source at adoption time. Do not infer that the fix shipped from an issue, merged pull request, branch, or remembered version number.
- Until a fixed stable release exists, an existing hand-written server may remain temporarily. A project that adopts `rmcp` earlier must provide and test its own bounded transport and record why waiting was not acceptable.
- Never trade an existing finite message-size or line-length limit for an SDK transport with unbounded buffering.
- Bound aggregate request work as well as individual frames. Cap in-flight requests and reject excess work rather than allowing an SDK to create an unbounded task, blocking-work, or response queue. Hold capacity through response transmission, not only until the handler returns, so a peer that stops reading cannot create unbounded buffered responses.

## Server shape

- Prefer an explicit server handler and a typed capability registry over hand-built JSON dispatch. The same typed engine contracts should drive CLI, SDK, and MCP surfaces.
- Implement the current stateless protocol and `server/discover`. Support the legacy initialization lifecycle only where deployed clients require dual-era compatibility; never claim support for an unimplemented protocol version.
- Keep local tools on newline-delimited stdio by default. Add Streamable HTTP only for a concrete consumer, with loopback binding, Origin validation, authentication, request limits, and transport-specific conformance tests.
- Return typed structured content alongside a useful text representation, accurate tool annotations, deterministic lists, required cache hints, and the protocol's distinction between JSON-RPC errors and model-correctable tool errors.

## Assurance

- Spawn the real server process in tests and drive both modern stateless discovery and any supported legacy initialization path with an official SDK client.
- Test malformed envelopes, invalid IDs, notifications, cancellation, EOF, stdout/stderr separation, oversized and unterminated input, in-flight overload behavior, unread-output backpressure, and every mutating tool's filesystem boundary and concurrency behavior.
- Keep pure domain tests below the protocol adapter. SDK conformance does not replace application-level result, permission, and safety tests.
