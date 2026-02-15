# find-cargo-toml

<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
![Crates.io Version](https://img.shields.io/crates/v/find-cargo-toml)
![Crates.io Total Downloads](https://img.shields.io/crates/d/find-cargo-toml)
![docs.rs](https://img.shields.io/docsrs/find-cargo-toml)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/betterhyq/find-cargo-toml)
![GitHub Repo stars](https://img.shields.io/github/stars/betterhyq/find-cargo-toml)
<!-- /automdrs -->

Find `Cargo.toml` (or a custom manifest file) by walking up the directory tree from a given path. Yields manifest paths from nearest to root—handy for locating package or workspace roots. Rust port of [find-package-json](https://www.npmjs.com/package/find-package-json).

## Installation

<!-- automdrs:cargo-add -->

```sh
cargo add find-cargo-toml
```

<!-- /automdrs -->

## Usage

**From current directory (default `Cargo.toml`):**

```rust
use find_cargo_toml::find;
use std::path::PathBuf;

for path in find(".", None::<PathBuf>, None) {
    println!("Found: {}", path.display());
}
```

**From a file path** (search starts at the file’s parent):

```rust
for path in find("src/lib.rs", None::<PathBuf>, None) {
    println!("Manifest: {}", path.display());
}
```

**Custom manifest filename:**

```rust
for path in find(".", None::<PathBuf>, Some("MyManifest.toml")) {
    println!("Found: {}", path.display());
}
```

**Explicit base for relative `input`:**

```rust
for path in find(".", Some(PathBuf::from("/some/base")), None) {
    println!("Found: {}", path.display());
}
```

**Convenience: start from current working directory:**

```rust
use find_cargo_toml::find_from_current_dir;

for path in find_from_current_dir(".", None) {
    println!("Found: {}", path.display());
}
```

## API

| Function | Description |
|----------|-------------|
| [`find(input, base, file_name)`](https://docs.rs/find-cargo-toml/*/find_cargo_toml/fn.find.html) | Walk up from `input` (directory or file). Resolve relative `input` against `base` (default: cwd). `file_name` defaults to `"Cargo.toml"`. Returns an iterator of manifest paths (nearest first). |
| [`find_from_current_dir(input, file_name)`](https://docs.rs/find-cargo-toml/*/find_cargo_toml/fn.find_from_current_dir.html) | Same as `find(input, None, file_name)`. |

- If `input` is a file, its parent directory is used as the start.
- Paths are normalized (`.` and `..` resolved) before walking.

## Contributing

<details>
<summary>Local development</summary>

1. Clone the repository.
2. Install the latest [Rust](https://rust-lang.org/).
3. Run tests: `cargo test`
4. Lint: `cargo clippy` · Format: `cargo fmt`

</details>

## Credits

Inspired by [find-package-json](https://www.npmjs.com/package/find-package-json).

## License

<!-- automdrs:contributors author="YONGQI" license="MIT" -->
Published under the [MIT](./LICENSE) license.
Made by [@YONGQI](https://github.com/betterhyq) 💛
<br><br>
<a href="https://github.com/betterhyq/find-cargo-toml/graphs/contributors">
<img src="https://contrib.rocks/image?repo=betterhyq/find-cargo-toml" />
</a>
<!-- /automdrs -->

<!-- automdrs:with-automdrs -->

---

_🛠️ auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_

<!-- /automdrs -->