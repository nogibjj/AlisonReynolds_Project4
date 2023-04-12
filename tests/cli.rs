use assert_cmd::Command;

#[test]
//a test to invoke the cli with an subcommand 'generate'
fn testing() {
    let mut cmd = Command::cargo_bin("big_data").unwrap();
    cmd.arg("predict").args([".8", "721"]);
    cmd.assert().success();
}
