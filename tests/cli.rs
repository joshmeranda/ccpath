use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn setup(files: &[&Path], dirs: &[&Path]) -> Result<TempDir, Box<dyn std::error::Error>> {
    let env_dir = tempfile::tempdir()?;
    let env_dir_path = env_dir.path();

    for file in files.iter() {
        let path = PathBuf::from(env_dir_path).join(file);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        File::create(path)?;
    }

    for dir in dirs {
        fs::create_dir_all(PathBuf::from(env_dir_path).join(dir))?;
    }
    Ok(env_dir)
}

#[test]
fn test_basename() -> Result<(), Box<dyn std::error::Error>> {
    let target_path = Path::new("Parent Dir").join("Some Child.txt");

    let dir = setup(&[&target_path], &[])?;

    let target_path = PathBuf::from(dir.path()).join(target_path);

    let expected_path = PathBuf::from(dir.path())
        .join("Parent Dir")
        .join("some_child.txt");

    let mut cmd = Command::cargo_bin("ccpath")?;
    cmd.args(&["--basename", "snake", target_path.to_str().unwrap()]);
    cmd.assert().success();

    assert!(expected_path.exists());

    Ok(())
}

#[test]
fn test_full_path() -> Result<(), Box<dyn std::error::Error>> {
    let target_path = Path::new("Parent Dir/Some Child.txt");

    let dir = setup(&[&target_path], &[])?;

    let target_path = PathBuf::from(dir.path()).join(target_path);

    let expected_path = PathBuf::from(dir.path())
        .join("parent_dir")
        .join("some_child.txt");

    let mut cmd = Command::cargo_bin("ccpath")?;
    cmd.args(&["--full-path", "--prefix", dir.path().to_str().unwrap(), "snake", target_path.to_str().unwrap()]);
    cmd.assert().success();

    assert!(expected_path.exists());

    Ok(())
}

#[test]
fn test_full_with_parents_pre_exist() -> Result<(), Box<dyn std::error::Error>> {
    let target_path = Path::new("Parent Dir/Some Child.txt");

    let dir = setup(&[&target_path], &[Path::new("parent_dir")])?;

    let target_path = PathBuf::from(dir.path()).join(target_path);

    let expected_path = PathBuf::from(dir.path())
        .join("parent_dir")
        .join("some_child.txt");

    let mut cmd = Command::cargo_bin("ccpath")?;
    cmd.args(&["--verbose", "--full-path", "--prefix", dir.path().to_str().unwrap(), "snake", target_path.to_str().unwrap()]);
    cmd.assert().success();

    assert!(expected_path.exists());

    Ok(())
}

#[test]
fn test_full_with_parents_no_exist() -> Result<(), Box<dyn std::error::Error>> {
    let target_path = Path::new("Parent Dir/Some Child.txt");

    let dir = setup(&[&target_path], &[])?;

    let target_path = PathBuf::from(dir.path()).join(target_path);

    let expected_path = PathBuf::from(dir.path())
        .join("parent_dir")
        .join("some_child.txt");

    let mut cmd = Command::cargo_bin("ccpath")?;
    cmd.args(&["--verbose", "--full-path", "--prefix", dir.path().to_str().unwrap(), "snake", target_path.to_str().unwrap()]);
    cmd.assert().success();

    assert!(expected_path.exists());

    Ok(())
}

// #[test]
// fn test_overwrite() -> Result<(), Box<dyn std::error::Error>> {}

// #[test]
// fn test_recursive() -> Result<(), Box<dyn std::error::Error>> {}

#[test]
fn test_no_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ccpath")?;

    cmd.arg("snake");

    cmd.assert().failure();

    Ok(())
}

#[test]
fn test_unsupported_convention() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ccpath")?;

    cmd.arg("unsupported convention").arg("/some/path");

    cmd.assert()
        .failure()
        .stderr(predicate::str::is_match("Unsupported naming convention '.*'").unwrap());

    Ok(())
}

#[test]
fn test_no_convention() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ccpath")?;

    cmd.assert().failure();

    Ok(())
}

#[test]
fn test_basename_mutually_exclusive_mode_group() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ccpath")?;

    cmd.arg("--basename").arg("full-path");

    cmd.assert().failure();

    Ok(())
}

#[test]
fn test_path_no_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ccpath")?;
    cmd.args(&["snake", "/path/does/not/exist"]);
    cmd.assert().failure();

    Ok(())
}