# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust application called "clock-floater" - currently in initial setup phase with minimal implementation.
Clock Floater is an GUI application written in rust with minimal design, running on MacOS.
Main functionality:
- Show a timer
- Set a countdown
- Alarm when time reach 0

## Build and Development Commands

- **Build**: `cargo build`
- **Run**: `cargo run`
- **Build (release)**: `cargo build --release`
- **Run tests**: `cargo test`
- **Check code**: `cargo check`
- **Format code**: `cargo fmt`
- **Lint**: `cargo clippy`

## Architecture

This is a new Rust project with standard Cargo project structure:
- `Cargo.toml` - Project manifest and dependencies
- `src/main.rs` - Application entry point
- `/target` - Build artifacts (gitignored)

The project currently uses Rust edition 2024 and has no external dependencies defined yet.
