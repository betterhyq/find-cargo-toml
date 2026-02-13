# find-cargo-toml

![Crates.io Version](https://img.shields.io/crates/v/find-cargo-toml)
![Crates.io Total Downloads](https://img.shields.io/crates/d/find-cargo-toml)
![docs.rs](https://img.shields.io/docsrs/find-cargo-toml)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/betterhyq/find-cargo-toml)
![GitHub Repo stars](https://img.shields.io/github/stars/betterhyq/find-cargo-toml)

`find-cargo-toml` finds `Cargo.toml` (or a custom manifest filename) by walking up the directory tree from a given path—a Rust port of the npm package [find-package-json](https://www.npmjs.com/package/find-package-json).

Use it when you need to locate the nearest Cargo manifest from a subdirectory or file path (e.g. in tools, editors, or build scripts).

## Installation

Add this crate with Cargo:

```bash
cargo add find-cargo-toml
```

## Usage

```rust
use find_cargo_toml::find;
use std::path::PathBuf;

// Find all Cargo.toml from current dir upward
for path in find(".", None::<PathBuf>, None) {
    println!("Found: {}", path.display());
}

// Start from a specific path (e.g. a source file)
for path in find("src/lib.rs", None::<PathBuf>, None) {
    println!("Manifest: {}", path.display());
}

// Use a custom manifest filename
for path in find(".", None::<PathBuf>, Some("MyManifest.toml")) {
    println!("Found: {}", path.display());
}

// Resolve relative input against an explicit base
for path in find(".", Some("/some/base"), None) {
    println!("Found: {}", path.display());
}
```

Convenience function when starting from the current working directory:

```rust
use find_cargo_toml::find_from_current_dir;

for path in find_from_current_dir(".", None) {
    println!("Found: {}", path.display());
}
```

## API

| Function | Description |
|----------|-------------|
| `find(input, base, file_name)` | Walk up from `input` (dir or file); resolve relative `input` against `base` (or cwd). Optional `file_name` (default `"Cargo.toml"`). Returns an iterator of manifest paths. |
| `find_from_current_dir(input, file_name)` | Same as `find(input, None, file_name)`—starts from the current directory. |

- If `input` is a file path, its parent directory is used as the starting directory.
- Paths are normalized (`.` and `..` are resolved) before walking.

## Contribution

<details>
  <summary>Local development</summary>

- Clone this repository
- Install the latest version of [Rust](https://rust-lang.org/)
- Run tests with `cargo test`

</details>

## Credits

`find-cargo-toml` is inspired by the npm package [find-package-json](https://www.npmjs.com/package/find-package-json), which finds `package.json` by walking up the directory tree.

## License

Published under the [MIT](./LICENSE) license.
Made by [@YONGQI](https://github.com/betterhyq) 💛
<br><br>
<a href="https://github.com/betterhyq/find-cargo-toml/graphs/contributors">
<img src="https://contrib.rocks/image?repo=betterhyq/find-cargo-toml" />
</a>
