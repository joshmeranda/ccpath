use std::fs::{self, File};
use std::path::{Path, PathBuf};

use assert_cmd::Command;

use predicates::prelude::predicate;

use tempfile::TempDir;

fn setup(files: &[&Path], dirs: &[&Path]) -> Result<TempDir, Box<dyn std::error::Error>> {
    let env_dir = tempfile::tempdir()?;
    let env_dir_path = env_dir.path();

    for file in files.iter() {
        let path = PathBuf::from(env_dir_path).join(file);

        if let Some(parent) = path.parent() {
            // println!("=== 011 '{}' ===", parent.display());
            fs::create_dir_all(parent)?;
        }

        // println!("=== 010 '{}' ---", path.clone().display());

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

    assert!(!target_path.exists());
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
    cmd.args(&[
        "--full-path",
        "--prefix",
        dir.path().to_str().unwrap(),
        "snake",
        target_path.to_str().unwrap(),
    ]);
    cmd.assert().success();

    assert!(!target_path.exists());
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
    cmd.args(&[
        "--verbose",
        "--full-path",
        "--prefix",
        dir.path().to_str().unwrap(),
        "snake",
        target_path.to_str().unwrap(),
    ]);
    cmd.assert().success();

    assert!(!target_path.exists());
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
    cmd.args(&[
        "--full-path",
        "--prefix",
        dir.path().to_str().unwrap(),
        "snake",
        target_path.to_str().unwrap(),
    ]);
    cmd.assert().success();

    assert!(!target_path.exists());
    assert!(expected_path.exists());

    Ok(())
}

#[test]
fn test_overwrite() -> Result<(), Box<dyn std::error::Error>> {
    let target_path = Path::new("Some File.txt");
    let existing_path = Path::new("some_file.txt");

    let dir = setup(&[&target_path, existing_path], &[])?;

    let target_path = dir.path().join(target_path);
    let existing_path = dir.path().join(existing_path);

    let mut cmd = Command::cargo_bin("ccpath")?;
    cmd.args(&[
        "--full-path",
        "--prefix",
        dir.path().to_str().unwrap(),
        "snake",
        target_path.to_str().unwrap(),
    ]);
    cmd.assert().success();

    assert!(!target_path.exists());
    assert!(existing_path.exists());

    Ok(())
}

#[test]
fn test_overwrite_no_clobber() -> Result<(), Box<dyn std::error::Error>> {
    let target_path = Path::new("Some File.txt");
    let existing_path = Path::new("some_file.txt");

    let dir = setup(&[&target_path, existing_path], &[])?;

    let target_path = dir.path().join(target_path);
    let existing_path = dir.path().join(existing_path);

    let mut cmd = Command::cargo_bin("ccpath")?;
    cmd.args(&[
        "--verbose",
        "--no-clobber",
        "--full-path",
        "--prefix",
        dir.path().to_str().unwrap(),
        "snake",
        target_path.to_str().unwrap(),
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "file '{}' already exists",
            existing_path.display()
        )));

    assert!(target_path.exists());
    assert!(existing_path.exists());

    Ok(())
}

#[test]
fn test_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let parent_dir = Path::new("Parent Dir");
    let child_dir = parent_dir.join("Child Dir");
    let child_file = parent_dir.join("Child File.txt");
    let grand_child_file = child_dir.join("Grand Child File.txt");
    let another_grand_child_file = child_dir.join("Another Grand Child File.txt");

    let dir = setup(&[child_file.as_path(), grand_child_file.as_path(), another_grand_child_file.as_path()], &[])?;

    let parent_dir = dir.path().join(parent_dir);
    let child_dir = dir.path().join(child_dir);
    let child_file = dir.path().join(child_file);
    let grand_child_file = dir.path().join(grand_child_file);
    let another_grand_child_file = dir.path().join(another_grand_child_file);

    let mut cmd = Command::cargo_bin("ccpath")?;
    cmd.args(&[
        "--recursive",
        "--prefix",
        dir.path().to_str().unwrap(),
        "snake",
        parent_dir.to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(!parent_dir.exists());
    assert!(!child_dir.exists());
    assert!(!child_file.exists());
    assert!(!grand_child_file.exists());
    assert!(!another_grand_child_file.exists());

    assert!(dir.path().join("parent_dir").exists());
    assert!(dir
        .path()
        .join("parent_dir")
        .join("child_file.txt")
        .exists());
    assert!(dir.path().join("parent_dir").join("child_dir").exists());
    assert!(dir
        .path()
        .join("parent_dir")
        .join("child_dir")
        .join("grand_child_file.txt")
        .exists());
    assert!(dir
        .path()
        .join("parent_dir")
        .join("child_dir")
        .join("another_grand_child_file.txt")
        .exists());

    Ok(())
}

#[test]
fn test_dir_no_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let parent_dir = Path::new("Parent Dir");
    let child_dir = parent_dir.join("Child Dir");
    let child_file = parent_dir.join("Child File.txt");
    let grand_child_file = child_dir.join("Grand Child File.txt");

    let dir = setup(&[child_file.as_path(), grand_child_file.as_path()], &[])?;

    let parent_dir = dir.path().join(parent_dir);
    let child_dir = dir.path().join(child_dir);
    let child_file = dir.path().join(child_file);
    let grand_child_file = dir.path().join(grand_child_file);

    let mut cmd = Command::cargo_bin("ccpath")?;
    cmd.args(&[
        "--prefix",
        dir.path().to_str().unwrap(),
        "snake",
        parent_dir.to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(!parent_dir.exists());
    assert!(!child_dir.exists());
    assert!(!child_file.exists());
    assert!(!grand_child_file.exists());

    assert!(dir.path().join("parent_dir").exists());
    assert!(dir
        .path()
        .join("parent_dir")
        .join("Child File.txt")
        .exists());
    assert!(dir.path().join("parent_dir").join("Child Dir").exists());
    assert!(dir
        .path()
        .join("parent_dir")
        .join("Child Dir")
        .join("Grand Child File.txt")
        .exists());

    Ok(())
}

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
