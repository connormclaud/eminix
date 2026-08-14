# Eminix Next Steps

This document outlines the roadmap from a working gap buffer to the full EEE 002 Provable Core. Each step produces a **runnable, interactive program** while teaching Rust concepts and building toward the event-driven actor architecture.

## Current State

| Component | Status | Notes |
|-----------|--------|-------|
| GapBuffer | ✅ Complete | Core data structure with 12 tests |
| Cargo.toml | ✅ Complete | Rust 2024 edition, crossterm 0.29 |
| src/lib.rs | ✅ Complete | Module structure in place |
| src/main.rs | ⏳ Skeleton | Raw mode enable/disable only |

---

## Phase 1: Get It Working

### Step 1 — Type and See

**Commit:** `feat: implement single-line editor with type and backspace`

**What to do:**
1. Rewrite `src/main.rs` to a working single-line editor
2. Enable raw mode, clear screen at startup
3. Loop: read key events with `crossterm::event::read()`
4. On `Char(c)`: insert into GapBuffer, re-render the line
5. On `Backspace`: call `buf.delete()`, re-render
6. On `Ctrl-C`: disable raw mode, exit cleanly
7. Re-render = clear line + print `buf.contents()` + position cursor

**You can run it:** Type characters and see them appear. Backspace works. Ctrl-C exits.

**Rust concepts:** `match` on `KeyCode` variants, `crossterm::execute!` macro, `loop`/`break`, raw terminal mode setup/teardown

**Files:** `src/main.rs`

---

### Step 2 — Arrow Keys and Cursor Movement

**Commit:** `feat: add arrow key cursor movement`

**What to do:**
1. Handle `Left`/`Right` arrow keys → `move_left()`/`move_right()`
2. Compute on-screen cursor position from buffer state (`start` field)
3. Position cursor with `crossterm::cursor::MoveTo`
4. Handle edge cases (cursor at start/end does nothing)

**You can run it:** Arrow keys move the cursor. Type in the middle of text. It feels like a real editor!

**Rust concepts:** more `match` arms, computing derived state, `cursor::MoveTo(col, row)`

**Files:** `src/main.rs`

---

## Phase 2: Introduce Events

### Step 3 — Event Enum

**Commit:** `feat: introduce Event and Priority enums`

**What to do:**
1. Create `src/event.rs` with:
   - `Event` enum: `KeyPress(u8)`, `Delete`, `CursorMove(Direction)`, `Shutdown`, `BufferUpdated`, `CursorMoved`
   - `Direction` enum: `Left`, `Right`
   - `Priority` enum: `Critical`, `Ephemeral`
   - `Event::priority() -> Priority` method
2. Wire into `src/lib.rs`
3. Refactor main.rs: crossterm events → `Event` → handle
4. Write unit tests for event priority classification

**You can run it:** Same behavior as before — but the code now uses your own Event type.

**Rust concepts:** enums with data, `#[derive(Debug, Clone, PartialEq)]`, `impl` blocks, `#[cfg(test)]` modules, converting between external and internal types

**Files:** `src/event.rs`, `src/lib.rs`, `src/main.rs`, `tests/event.rs`

---

### Step 4 — Channel Split

**Commit:** `feat: decouple input reading with mpsc channel`

**What to do:**
1. Create `src/channel.rs`
2. Spawn a thread (`std::thread::spawn`) that reads crossterm events and sends `Event`s over `std::sync::mpsc`
3. Main loop receives from the channel: `recv()` → handle → render
4. Write tests for channel message ordering

**You can run it:** Same editor, but input reading now runs in its own thread!

**Rust concepts:** `std::sync::mpsc::channel()`, `Send` trait, `thread::spawn`, `move` closures, ownership transfer across thread boundaries

**Files:** `src/channel.rs`, `src/lib.rs`, `src/main.rs`

---

## Phase 3: Go Async

### Step 5 — Tokio and Async Channels

**Commit:** `feat: convert to async with tokio runtime`

**What to do:**
1. Add `tokio` (full features) and `futures` to `Cargo.toml`
2. Replace `std::sync::mpsc` with `tokio::sync::mpsc` (bounded)
3. Replace `thread::spawn` with `tokio::spawn`
4. Add `#[tokio::main]` to main
5. Use `crossterm::event::EventStream` for async input reading
6. Write async tests with `#[tokio::test]`

**You can run it:** Same editor, now running on the tokio async runtime!

**Rust concepts:** `async`/`await`, `tokio::sync::mpsc`, `#[tokio::main]`, `futures::StreamExt`, `.await` on channel operations, `Send` + `Sync` bounds

**Files:** `Cargo.toml`, `src/channel.rs`, `src/main.rs`, `tests/channel.rs`

---

### Step 6 — Ephemeral Broadcast Channel

**Commit:** `feat: add broadcast channel for render notifications`

**What to do:**
1. Add `EphemeralBus` using `tokio::sync::broadcast`
2. After handling a buffer mutation, publish `BufferUpdated` on the bus
3. Rendering logic subscribes to the bus instead of being called directly
4. Handle `RecvError::Lagged` gracefully (just re-render everything)

**You can run it:** Same editor, but rendering is now triggered by events on a broadcast channel!

**Rust concepts:** `tokio::sync::broadcast`, `Clone` for subscribers, `RecvError` variant matching, publish/subscribe pattern

**Files:** `src/channel.rs`, `src/main.rs`

---

## Phase 4: Actor Model

### Step 7 — Actor Trait and BufferActor

**Commit:** `feat: implement Actor trait and BufferActor`

**What to do:**
1. Create `src/actor.rs` with `trait Actor` (`async fn handle(&mut self, event: Event)`)
2. Create `Mailbox` struct wrapping a bounded channel receiver
3. Create `src/buffer_actor.rs`: wraps `GapBuffer`, handles `KeyPress`/`Delete`/`CursorMove`, emits `BufferUpdated`/`CursorMoved` on the ephemeral bus
4. Write tests: send events to BufferActor, verify buffer state

**You can run it:** Same editor, but the buffer is now managed by an actor receiving messages!

**Rust concepts:** async trait methods, struct composition (actor owns a `GapBuffer`), `tokio::select!`, the actor pattern

**Files:** `src/actor.rs`, `src/buffer_actor.rs`, `src/lib.rs`, `tests/actor.rs`

---

### Step 8 — InputReader and Renderer Actors

**Commit:** `feat: implement full 3-actor pipeline`

**What to do:**
1. Create `src/input_reader.rs`: owns `EventStream`, converts terminal input to `Event`, sends to BufferActor's mailbox
2. Create `src/renderer.rs`: subscribes to ephemeral bus, renders buffer state to terminal
3. Rewrite `main.rs` to spawn all three actors with `tokio::spawn`
4. Implement graceful shutdown: `Shutdown` event propagates to all actors

**You can run it:** The full EEE 002 architecture! Input → BufferActor → Renderer, all as separate async actors.

**Rust concepts:** `tokio::spawn`, `JoinHandle`, graceful shutdown with `tokio::signal`, coordinating multiple async tasks

**Files:** `src/input_reader.rs`, `src/renderer.rs`, `src/lib.rs`, `src/main.rs`

---

### Step 9 — Priority Mailbox

**Commit:** `feat: implement priority-aware actor mailbox`

**What to do:**
1. Enhance `Mailbox` with both a critical channel and an ephemeral subscription
2. Implement priority-aware receive: always drain critical events before ephemeral
3. Write a test proving critical events are processed first under load

**You can run it:** Same editor, but now keystrokes are guaranteed never dropped even under pressure.

**Rust concepts:** `tokio::select!` with `biased`, priority queues, backpressure behavior

**Files:** `src/actor.rs`, `tests/actor.rs`

---

## Phase 5: Prove It

### Step 10 — Benchmarks, Integration Tests, and Docs

**Commit:** `feat: add benchmarks and documentation (EEE 002 complete)`

**What to do:**
1. Add `criterion` to `Cargo.toml` `[dev-dependencies]`
2. Create `benches/` with benchmarks:
   - Gap buffer: 10k sequential inserts, insert at random positions
   - Channel round-trip: ephemeral and critical
   - End-to-end: simulated keypress → buffer update (target: p99 < 1ms)
3. Add integration tests for the full actor pipeline
4. Write `README.md` with architecture overview and benchmark results
5. Verify: `cargo test`, `cargo clippy`, `cargo bench` all pass

**You can run it:** `cargo bench` prints latency histograms. You can see the p99 numbers!

**Rust concepts:** `criterion` crate, benchmarking async code, integration test organization, technical documentation

**Files:** `Cargo.toml`, `benches/`, `tests/integration.rs`, `README.md`

---

## EEE 002 Definition of Done

- [ ] `cargo test` — all tests green
- [ ] `cargo clippy` — no warnings
- [ ] `cargo bench` — latency histograms produced
- [ ] `cargo run` — accepts keystrokes, displays them, exits on Ctrl-C
- [ ] `README.md` — documents architecture, build instructions, benchmark results

## Dopamine Map

| Step | What You See |
|------|-------------|
| 1 | Characters appear as you type! |
| 2 | Arrow keys move the cursor! |
| 3 | Same thing + cleaner code + tests pass |
| 4 | Same thing + input runs in its own thread |
| 5 | Same thing + now it's async! |
| 6 | Rendering is event-driven! |
| 7 | Buffer is managed by an actor! |
| 8 | Full 3-actor architecture running! |
| 9 | Priority system protects keystrokes! |
| 10 | Benchmark numbers prove it's fast! |
