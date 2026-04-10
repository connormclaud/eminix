# Eminix Next Steps

This document outlines the recommended next steps for building the Eminix text editor. Each step is designed to be small enough to commit individually while teaching important Rust concepts.

## Current State

| Component | Status | Notes |
|-----------|--------|-------|
| GapBuffer | ✅ Complete | Core data structure implemented and tested |
| Cargo.toml | ✅ Complete | Rust 2024 edition configured |
| src/lib.rs | ✅ Complete | Module structure in place |
| src/main.rs | ⏳ Empty | Ready for implementation |

---

## Step 1: Add Crossterm Crate

**Commit Message:** `feat: add crossterm dependency for terminal control`

**What to do:**
1. Add `crossterm = "0.27"` to `Cargo.toml` under `[dependencies]`
2. Run `cargo update` to fetch the crate
3. Verify it compiles with `cargo check`

**What you'll learn:**
- Adding external dependencies
- Cargo.toml structure
- Dependency resolution

**Files to modify:**
- `Cargo.toml`

---

## Step 2: Raw Input Loop

**Commit Message:** `feat: implement raw terminal input loop`

**What to do:**
1. Import `crossterm` event handling in `src/main.rs`
2. Enable raw mode at startup
3. Create a loop that polls for key events
4. Insert character keystrokes into the GapBuffer

**What you'll learn:**
- Raw terminal mode setup/teardown
- Event polling with `crossterm`
- Matching `KeyCode` variants
- Basic async patterns

**Files to modify:**
- `src/main.rs`

**Key concepts:**
- `crossterm::terminal::enable_raw_mode()` / `disable_raw_mode()`
- `crossterm::event::poll()` for non-blocking input
- `crossterm::event::read()` for blocking input
- `KeyCode::Char` for regular characters
- `KeyCode::Enter`, `KeyCode::Backspace`, `KeyCode::Left`, `KeyCode::Right` for special keys

---

## Step 3: Rendering Output

**Commit Message:** `feat: render buffer contents to terminal`

**What to do:**
1. Clear the terminal screen at startup
2. Render the GapBuffer contents to the terminal
3. Update the cursor position based on buffer state
4. Handle terminal resizing gracefully

**What you'll learn:**
- Terminal cursor positioning
- Screen clearing and refresh
- Basic TUI rendering patterns

**Files to modify:**
- `src/main.rs`

**Key concepts:**
- `crossterm::cursor::MoveTo(x, y)`
- `crossterm::style::Print(content)`
- `crossterm::terminal::Clear(terminal::ClearType::All)`
- `crossterm::execute!()` macro for terminal commands

---

## Step 4: Handling Special Keys

**Commit Message:** `feat: handle backspace and arrow keys`

**What to do:**
1. Detect backspace key and call `GapBuffer::delete()`
2. Detect arrow keys and call `GapBuffer::move_left()` / `move_right()`
3. Handle Enter key (newline insertion)
4. Handle Ctrl-C for graceful shutdown

**What you'll learn:**
- Event-driven input handling
- State machine patterns for key handling
- Graceful shutdown with signal handling

**Files to modify:**
- `src/main.rs`

**Key concepts:**
- `KeyCode::Backspace` → `buf.delete()`
- `KeyCode::Left` → `buf.move_left()`
- `KeyCode::Right` → `buf.move_right()`
- `KeyCode::Enter` → `buf.insert(b'\n')`
- `Ctrl-C` → break loop and disable raw mode

---

## Step 5: Integration Tests

**Commit Message:** `test: add integration tests for input loop`

**What to do:**
1. Create `tests/integration.rs` with end-to-end tests
2. Test complete keystroke sequences
3. Verify buffer state after various operations
4. Test edge cases (empty buffer, cursor at boundaries)

**What you'll learn:**
- Integration testing patterns
- Testing async code
- Test isolation and cleanup

**Files to create:**
- `tests/integration.rs`

**Key concepts:**
- `#[test]` for test functions
- `assert_eq!()` for assertions
- Test setup and teardown
- Mocking terminal input for tests

---

## Step 6: Documentation

**Commit Message:** `docs: add README.md with project overview`

**What to do:**
1. Create `README.md` with project description
2. Document how to build and run the project
3. Include architecture overview
4. Add benchmark results from Step 2

**What you'll learn:**
- Technical documentation best practices
- README structure for open source projects
- Linking documentation to code

**Files to create:**
- `README.md`

**Key concepts:**
- Markdown formatting
- Project structure documentation
- Getting started instructions
- Architecture diagrams (optional)

---

## Summary

By following these 6 steps, you'll have:
- ✅ A working text editor with raw terminal input
- ✅ Proper cursor movement and text editing
- ✅ Clean separation of concerns
- ✅ Test coverage for core functionality
- ✅ Documentation for future contributors

Each step is designed to be commitable independently, allowing you to track progress and revert if needed.
