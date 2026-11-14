## What and why

<!-- What does this change, and why? Link the issue if there is one. -->

## How it was tested

<!-- Tests added, and how you checked the behaviour (CLI, MCP, screenshots). -->

## Checklist

- [ ] `cargo xtask ci` passes (fmt, clippy, tests, layers, assets, wasm).
- [ ] No panics in shipped code: no `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!` or
      `unimplemented!` outside tests, and no unchecked indexing on data from files, parameters or
      messages. Errors are returned as `Result` and shown to the user ([AGENTS.md](https://github.com/getnodalia/trama/blob/main/AGENTS.md)).
- [ ] Clean-room: nothing copied from Figma or other products, or from GPL/AGPL code ([AGENTS.md](https://github.com/getnodalia/trama/blob/main/AGENTS.md)).
- [ ] Every new asset has a row in `ATTRIBUTION.md`.

## Compatibility and release impact

<!-- Note affected platforms, migrations or distribution changes when applicable. -->
