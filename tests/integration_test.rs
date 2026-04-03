//! Integration tests for the `typora-cli` binary.
//!
//! All tests pass `--dry-run` so the binary performs file creation but never
//! tries to spawn Typora, which may not be installed in CI.

use std::process::Command;

fn typora(dir: &std::path::Path, args: &[&str]) -> std::process::Output {
    let bin = env!("CARGO_BIN_EXE_typora-cli");
    Command::new(bin)
        .arg("--dry-run")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to execute typora-cli binary")
}

// ---------------------------------------------------------------------------
// Sentinel / open-current-dir cases
// ---------------------------------------------------------------------------

#[test]
fn no_argument_exits_successfully() {
    let dir = tempfile::tempdir().unwrap();
    assert!(typora(dir.path(), &[]).status.success());
}

#[test]
fn dot_argument_exits_successfully() {
    let dir = tempfile::tempdir().unwrap();
    assert!(typora(dir.path(), &["."]).status.success());
}

// ---------------------------------------------------------------------------
// File creation — exact name, no auto-suffix
// ---------------------------------------------------------------------------

#[test]
fn creates_file_with_exact_name_given() {
    let dir = tempfile::tempdir().unwrap();
    let out = typora(dir.path(), &["notes"]);
    assert!(out.status.success());
    // File is created with exactly the name provided — no .md appended.
    assert!(dir.path().join("notes").is_file());
    assert!(
        !dir.path().join("notes.md").exists(),
        "must not auto-append .md"
    );
}

#[test]
fn creates_md_file_when_user_explicitly_provides_extension() {
    let dir = tempfile::tempdir().unwrap();
    let out = typora(dir.path(), &["notes.md"]);
    assert!(out.status.success());
    assert!(dir.path().join("notes.md").is_file());
    assert!(!dir.path().join("notes.md.md").exists());
}

#[test]
fn creates_file_with_arbitrary_extension() {
    let dir = tempfile::tempdir().unwrap();
    let out = typora(dir.path(), &["readme.txt"]);
    assert!(out.status.success());
    assert!(dir.path().join("readme.txt").is_file());
}

#[test]
fn creates_file_with_spaces_in_name() {
    let dir = tempfile::tempdir().unwrap();
    let out = typora(dir.path(), &["my notes"]);
    assert!(out.status.success());
    assert!(dir.path().join("my notes").is_file());
}

// ---------------------------------------------------------------------------
// Existing file is not overwritten
// ---------------------------------------------------------------------------

#[test]
fn does_not_overwrite_existing_file() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("existing.md");
    std::fs::write(&target, b"original content").unwrap();

    let out = typora(dir.path(), &["existing.md"]);
    assert!(out.status.success());
    assert_eq!(std::fs::read(&target).unwrap(), b"original content");
}

// ---------------------------------------------------------------------------
// Absolute paths
// ---------------------------------------------------------------------------

#[test]
fn absolute_path_creates_new_file() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("new.md");
    assert!(!target.exists());

    let out = typora(dir.path(), &[target.to_str().unwrap()]);
    assert!(out.status.success());
    assert!(target.is_file());
}

#[test]
fn absolute_path_does_not_overwrite_existing_file() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("keep.md");
    std::fs::write(&target, b"keep me").unwrap();

    let out = typora(dir.path(), &[target.to_str().unwrap()]);
    assert!(out.status.success());
    assert_eq!(std::fs::read(&target).unwrap(), b"keep me");
}

// ---------------------------------------------------------------------------
// Nested directories are created automatically
// ---------------------------------------------------------------------------

#[test]
fn creates_missing_parent_directories() {
    let dir = tempfile::tempdir().unwrap();
    let out = typora(dir.path(), &["docs/work/meeting.md"]);
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(dir.path().join("docs/work/meeting.md").is_file());
}

// ---------------------------------------------------------------------------
// Parent-directory relative paths
// ---------------------------------------------------------------------------

#[test]
fn parent_dir_relative_path_creates_file_correctly() {
    let root = tempfile::tempdir().unwrap();
    let child = root.path().join("child");
    std::fs::create_dir(&child).unwrap();

    let out = typora(&child, &["../sibling.md"]);
    assert!(out.status.success());
    assert!(root.path().join("sibling.md").is_file());
}
