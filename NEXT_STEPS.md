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
