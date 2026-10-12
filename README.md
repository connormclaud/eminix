# Eminix

A text-based operating environment written in Rust, where text is the interface for everything.

> **Status:** a toy project, intentionally overdesigned so that it is never finished. It is a vehicle for learning Rust. The journey is the point.

## Planned overdesign

- Decentralized, event-driven actor model with a microkernel philosophy
- UI and core daemon run as separate processes and talk over IPC, so a UI crash can't corrupt editor state
- Critical events (user input) travel on lossless channels. Ephemeral events (UI updates) may be dropped under load.
- Your environment, your rules: if a user can build it, the core shouldn't

See [`eee/`](eee/) for the design proposals, starting with `EEE 001.md`.

## Current state

Nothing is ready. There is a lone gap buffer (`src/gap_buffer.rs`) and a few tests, and that's it.

```bash
cargo test
```

## Apache License 2.0

See [LICENSE](LICENSE).
