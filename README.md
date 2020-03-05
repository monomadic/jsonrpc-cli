jsonrpc-cli
============

Simple cli tool for testing output from JSON RPC 2.0 socket services. Pipe the output into a json tool like jq, [gron](https://github.com/TomNomNom/gron), [jid](https://github.com/simeji/jid) or peco for extra coolness.

Think of it like curl, but for JSON RPC2.0 over websockets.

### CLI instructions

```bash
cargo run -- ws://127.0.0.1:2222 method data.json
```

Another example with output piped to [jid](https://github.com/simeji/jid).
```bash
cargo run -- ws://127.0.0.1:7779 serialise rust.json | jid
```

Or jq (for pretty printing) and peco (for filtering).
```bash
cargo run -- ws://127.0.0.1:7779 parse rust.json | jq | peco
```
