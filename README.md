json-rpc-get
============

Simple wget-like tool for testing output from JSON RPC 2.0 socket services. Pipe the output into a json tool like jq for extra coolness.

### CLI instructions

```bash
cargo run -- ws://127.0.0.1:2222 method data.json
```
