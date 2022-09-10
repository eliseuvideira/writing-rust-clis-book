use std::process::Command;

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("main").unwrap();

    cmd.assert().success();
}
