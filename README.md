# Notes

# CLIs

- Cargo is the package manager like NPM
- Rustup is the toolchain manager
- Rustc is the compiler

```bash
cargo --version
```
```bash
rustup --version
```
```bash
rustc --version
```

## Initializing a new project
Wrap the bootstrap of project into a new folder named as "project-name"
```bash
cargo new project-name
```

Generate the bootstrap of project inside the current directory
```bash
cargo init
```

## Development 
Build a debug version for development and run it
```bash
cargo run
```

Only build a debug version
```bash
cargo build
```

Build the production version
```
cargo build --release
```