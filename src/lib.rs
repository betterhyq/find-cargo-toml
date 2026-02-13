//! Find `Cargo.toml` (or a custom filename) by walking up the directory tree.
//!
//! Inspired by the npm package [find-package-json](https://www.npmjs.com/package/find-package-json).

use std::path::{Path, PathBuf};

/// Iterator that walks up from a directory and yields each path where the manifest file exists.
pub struct FindIter {
    current: Option<PathBuf>,
    file_name: String,
}

impl Iterator for FindIter {
    type Item = PathBuf;

    fn next(&mut self) -> Option<PathBuf> {
        while let Some(ref dir) = self.current {
            let candidate = dir.join(&self.file_name);
            if candidate.is_file() {
                let result = candidate;
                self.current = dir.parent().map(PathBuf::from);
                return Some(result);
            }
            self.current = dir.parent().map(PathBuf::from);
        }
        None
    }
}

/// Find manifest files (default `Cargo.toml`) by walking up from `input`.
///
/// * `input` – Path to start searching from (directory or file; if file, its parent is used).
/// * `base` – Optional base path to resolve `input` against when `input` is relative.
///   If `None`, uses the current working directory.
/// * `file_name` – Manifest filename to look for (default: `"Cargo.toml"`).
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use find_cargo_toml::find;
///
/// for path in find(".", None::<PathBuf>, None) {
///     println!("Found: {}", path.display());
/// }
/// ```
pub fn find<P, Q>(
    input: P,
    base: Option<Q>,
    file_name: Option<&str>,
) -> FindIter
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let file_name = file_name.unwrap_or("Cargo.toml").to_string();
    let base: PathBuf = base
        .map(|b| b.as_ref().to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let start: PathBuf = if input.as_ref().is_absolute() {
        input.as_ref().to_path_buf()
    } else {
        base.join(input.as_ref())
    };
    let start_normalized = normalize_path(&start);
    let start_dir = if start_normalized.is_file() {
        start_normalized
            .parent()
            .map(PathBuf::from)
            .unwrap_or(start_normalized)
    } else {
        start_normalized
    };

    FindIter {
        current: Some(start_dir),
        file_name,
    }
}

/// Same as [`find`], but always starts from the current directory (no base).
pub fn find_from_current_dir<P>(input: P, file_name: Option<&str>) -> FindIter
where
    P: AsRef<Path>,
{
    find(input, None::<PathBuf>, file_name)
}

/// Normalize a path by resolving `.` and `..` components.
fn normalize_path(path: &Path) -> PathBuf {
    path.components().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn find_yields_nothing_when_no_cargo_toml() {
        let tmp = std::env::temp_dir().join("find_cargo_toml_test_empty");
        let _ = fs::create_dir_all(&tmp);
        let count = find(&tmp, None::<PathBuf>, None).count();
        let _ = fs::remove_dir_all(&tmp);
        assert_eq!(count, 0);
    }

    #[test]
    fn find_yields_path_when_cargo_toml_in_dir() {
        let tmp = std::env::temp_dir().join("find_cargo_toml_test_with");
        let _ = fs::create_dir_all(&tmp);
        let manifest = tmp.join("Cargo.toml");
        let _ = fs::File::create(&manifest).and_then(|mut f| f.write_all(b"[package]"));
        let collected: Vec<_> = find(&tmp, None::<PathBuf>, None).collect();
        let _ = fs::remove_file(manifest);
        let _ = fs::remove_dir_all(&tmp);
        assert_eq!(collected.len(), 1);
        assert!(collected[0].ends_with("Cargo.toml"));
    }

    #[test]
    fn find_respects_custom_file_name() {
        let tmp = std::env::temp_dir().join("find_cargo_toml_test_custom");
        let _ = fs::create_dir_all(&tmp);
        let custom = tmp.join("MyManifest.toml");
        let _ = fs::File::create(&custom).and_then(|mut f| f.write_all(b"[package]"));
        let collected: Vec<_> = find(&tmp, None::<PathBuf>, Some("MyManifest.toml")).collect();
        let _ = fs::remove_file(custom);
        let _ = fs::remove_dir_all(&tmp);
        assert_eq!(collected.len(), 1);
        assert!(collected[0].ends_with("MyManifest.toml"));
    }
}
