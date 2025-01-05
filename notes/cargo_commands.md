# Common Cargo Commands Explained

In Rust, **`cargo`** is the official build system and package manager. This document explains commonly used `cargo` commands: `cargo build`, `cargo run`, and `cargo check`. These commands are fundamental to Rust development and streamline the workflow for building, testing, and running code.

## 1. `cargo build`

The `cargo build` command compiles your project and its dependencies into an executable or library.

### Key Features:
- **Compilation**: Produces a binary or library file from your Rust source code.
- **Default Target**: Debug build (`target/debug`).
- **Creates Artifacts**: The compiled binary is stored in the `target` directory.

### Usage:
```bash
cargo build
```
- Runs the compilation process for the project.

### Example:
```bash
cargo build
```
Output:
```plaintext
Compiling my_project v0.1.0 (/path/to/project)
Finished dev [unoptimized + debuginfo] target(s) in 1.23s
```

### Debug vs. Release:
- **Debug Build**: Default, includes debug information but is unoptimized.
  ```bash
  cargo build
  ```
- **Release Build**: Optimized for performance but excludes debug info.
  ```bash
  cargo build --release
  ```
  Output binary will be located in `target/release`.

---

## 2. `cargo run`

The `cargo run` command compiles and runs your Rust project in one step. It is commonly used during development to test functionality quickly.

### Key Features:
- Combines `cargo build` and executing the compiled binary.
- Automatically re-compiles if changes are detected.

### Usage:
```bash
cargo run
```
- Compiles the project if necessary and executes the resulting binary.

### Example:
```bash
cargo run
```
Output:
```plaintext
Compiling my_project v0.1.0 (/path/to/project)
Finished dev [unoptimized + debuginfo] target(s) in 1.23s
Running `target/debug/my_project`
Hello, world!
```

### Debug vs. Release:
- **Default**: Runs the debug version.
- **Release Mode**: Run the optimized version:
  ```bash
  cargo run --release
  ```

### Arguments:
- Pass arguments to your program using `--`:
  ```bash
  cargo run -- arg1 arg2
  ```
  Example:
  ```bash
  cargo run --input file.txt
  ```

---

## 3. `cargo check`

The `cargo check` command checks your code for errors without producing a binary. It is significantly faster than `cargo build` since it skips the compilation step.

### Key Features:
- **Syntax and Type Checking**: Ensures your code is correct without compiling it into a binary.
- **Faster Feedback**: Useful during development when iterating on code.

### Usage:
```bash
cargo check
```

### Example:
```bash
cargo check
```
Output:
```plaintext
Checking my_project v0.1.0 (/path/to/project)
Finished dev [unoptimized + debuginfo] target(s) in 0.43s
```

### When to Use:
- Use `cargo check` for quick checks during development.
- Use `cargo build` when you need a compiled binary.

---

## Summary
| Command        | Purpose                                         | Key Features                                   |
|----------------|-------------------------------------------------|-----------------------------------------------|
| `cargo build`  | Compiles the project into a binary or library. | Debug (`target/debug`) or release (`--release`). |
| `cargo run`    | Compiles and runs the project in one step.     | Combine build and execution, pass arguments.   |
| `cargo check`  | Checks code for errors without compiling.      | Faster feedback for syntax and type checking. |