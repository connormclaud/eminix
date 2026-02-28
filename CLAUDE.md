# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build          # Build debug
cargo run            # Run
cargo test           # Run all tests
cargo test <name>    # Run a single test by name
cargo clippy         # Lint
cargo fmt --check    # Check formatting
```

## Project Overview

Eminix is a modern, performance-oriented, extensible text-based operating environment written in Rust, where text is the interface for everything. It targets the "advanced casual key presser" — users who want a single, powerful, keyboard-driven environment for diverse workflows (writing, research, file management, task tracking, and more). Think the vision of Emacs rebuilt on a modern foundation.

Uses Rust edition 2024 with tokio for async runtime.

## Architecture

The design follows a **decentralized, event-driven, actor model** with a microkernel philosophy. Key architectural docs live in `eee/` as "EEE" (Eminix Enhancement EEE) proposals — start with `eee/EEE 001.md` for the full specification.

### Core Concepts

- **Event Priority System**: Events are either `Critical` (must never be dropped, e.g. user input) or `Ephemeral` (can be dropped under load, e.g. UI updates). This is the central QoS mechanism — see `src/events.rs`.
- **EventBus** (`src/event_bus.rs`): Broadcast channel (tokio MPMC) for **ephemeral events only**. Critical events use separate lossless bounded channels with backpressure. The bus panics if you try to publish a Critical event on it.
- **Process Isolation**: UI runs as a separate process from the core daemon, communicating via IPC. A UI crash cannot corrupt editor state.
- **Actor Isolation**: Components within the core are isolated actors with their own mailboxes and priority queues.

### Design Principles (The Zen of Eminix)

1. Text is the universal interface — text is not what you edit, it is how you interact with everything
2. Your environment, your rules — if something can be built by the user, it should not be built into the core
3. Flow is sacred — the system must never interrupt your thought ("hangs-free" principle)
4. Context shapes the interface — the environment reshapes itself around the work at hand
5. Simple parts, combined freely — each component does one thing well, power emerges from composition
