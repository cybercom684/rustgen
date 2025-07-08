# rustgen

A Rust command-line tool for automatically generating modular Rust projects and modules with automatic management of `mod.rs` files.

---

## Features

- Create new Rust library projects with a standard folder structure (`models`, `traits`, `services`)
- Add modules with automatic updates to `mod.rs`
- Create services with automatic stub implementations for traits
- Simple CLI interface using `clap`
- Perfect for fast, structured Rust projects and libraries

---

## Installation

**Build and run locally:**

```bash
git clone https://github.com/yourusername/rustgen.git
cd rustgen
cargo build
cargo run -- --help
```

## Usage
**Create a new library project**
```bash
cargo run -- new-lib <project_name>
```
Creates the *<project_name>* project with the standard structure.

<br>

**Add a module**
```bash
cargo run -- add-module <module_path> --project <project_name>
```

**Add a service**
```bash
cargo run -- add-service <service_name> --project <project_name> --trait-name <TraitName>

```
<br>

Example:
```bash
cargo run new-lib example
cargo run add-module models/user --project example
cargo run add-service demo_service --project example --trait-name demoRepository
```