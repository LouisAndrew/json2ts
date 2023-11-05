use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("json2ts")?;

    let path = "test/file/doesnt/exist";
    cmd.arg("foobar").args(["--path", path]);
    cmd.assert().failure().stderr(predicate::str::contains(
        "Error reading `".to_owned() + path,
    ));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("json2ts")?;
    cmd.arg("test")
        .args(["--path", file.path().to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}
