# Cargo Project Structure

In a Rust project, the **`Cargo`** structure organizes your code and metadata to streamline development. Here's an explanation of the components, particularly focusing on the `src` folder and how Rust projects are structured.

## Overview of a Cargo Project
When you create a Rust project using `cargo new`, Cargo initializes a directory with the following structure:

### Binary Project Example
```plaintext
my_project/
├── Cargo.toml
├── Cargo.lock
└── src/
    └── main.rs
```

### Library Project Example
```plaintext
my_library/
├── Cargo.toml
├── Cargo.lock
└── src/
    └── lib.rs
```

### Breakdown of Components

#### 1. `Cargo.toml`
- **Purpose**: This file contains the project metadata and dependency declarations.
- **Example**:
  ```toml
  [package]
  name = "my_project"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  serde = "1.0"
  ```

#### 2. `Cargo.lock`
- **Purpose**: Tracks the exact versions of dependencies used in the project, ensuring reproducible builds.
- **When Created**: Automatically generated after the first build.

#### 3. `src` Folder
- **Purpose**: Contains the source code files for your project.

### Default Files in the `src` Folder

#### For a Binary Project
```plaintext
src/
└── main.rs
```
- **`main.rs`**: The entry point for the binary project, containing the `main` function.
  ```rust
  fn main() {
      println!("Hello, world!");
  }
  ```

#### For a Library Project
```plaintext
src/
└── lib.rs
```
- **`lib.rs`**: The root module for the library, defining its public API.
  ```rust
  pub fn greet() {
      println!("Hello from the library!");
  }
  ```

## Organizing Code in the `src` Folder
As your project grows, you can organize your code using **modules**.

### Adding Modules
You can split functionality into separate files and declare them as modules.

#### Example with Additional Files
```plaintext
src/
├── main.rs
├── utils.rs
```
- **`main.rs`**:
  ```rust
  mod utils;

  fn main() {
      utils::say_hello();
  }
  ```
- **`utils.rs`**:
  ```rust
  pub fn say_hello() {
      println!("Hello from utils!");
  }
  ```

### Using Subdirectories for Modules
You can create submodules using folders.

#### Example with Nested Modules
```plaintext
src/
├── main.rs
└── utils/
    ├── mod.rs
    ├── math.rs
```
- **`main.rs`**:
  ```rust
  mod utils;

  fn main() {
      utils::math::add(2, 3);
  }
  ```
- **`utils/mod.rs`**:
  ```rust
  pub mod math;
  ```
- **`utils/math.rs`**:
  ```rust
  pub fn add(a: i32, b: i32) -> i32 {
      a + b
  }
  ```

## Summary
- **`Cargo.toml`**: Manages metadata and dependencies.
- **`Cargo.lock`**: Locks dependency versions.
- **`src` Folder**: Contains the core Rust source code, starting with `main.rs` (for binaries) or `lib.rs` (for libraries).
- **Modules**: Use additional files and directories in `src` to organize code into reusable and logical components.