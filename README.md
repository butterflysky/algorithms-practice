# Algorithms Practice

This repository contains a collection of algorithmic exercises and solutions implemented in various programming languages. It is organized to help me practice, learn, and master fundamental algorithms and data structures through hands-on coding.

## Structure

- `exercises/` — Contains subdirectories for each algorithmic problem or topic, with language-specific folders (e.g., `rust/`).
- Each exercise will eventually include:
  - Problem description (in code comments or a local README)
  - One or more implementations
  - Unit tests and/or property-based tests

## Languages Used
- Rust (primary)
- Python

## How to Use
1. Clone the repository:
   ```sh
   git clone https://github.com/butterflysky/algorithms-practice.git
   cd algorithms-practice
   ```
2. Navigate to an exercise and follow the instructions in its subdirectory.
3. For Rust exercises:
   ```sh
   cd exercises/<exercise-name>/rust
   cargo test
   ```
4. To add a new Rust crate, use the Cargo workspace setup:
   ```sh
   cargo workspaces create --lib --name <new-exercise-name> exercises/<new-exercise-name>/rust
   # or for a binary crate:
   cargo workspaces create --bin --name <new-exercise-name> exercises/<new-exercise-name>/rust
   # The glob in workspace members in exercises/Cargo.toml will capture the new crate automatically
   ```
   This repository uses a Cargo workspace, so new crates should be created with `cargo workspaces create`, rather than using `cargo init` inside an existing directory.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.
