
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn native_emits_jsonl() {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "-p", "runner_native", "--quiet", "--", "../sample-data/sample.pgm"])
       .current_dir("runtime");
    let out = cmd.assert().success().get_output().stdout.clone();
    let text = String::from_utf8(out).unwrap();
    assert!(text.contains(""event":"image.analyzed""));
    assert!(text.contains(""payload""));
}
