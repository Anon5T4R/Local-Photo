//! The suite's agent-native mould in Local Photo: `commands --search`, `run --script` with a JSON
//! array, and `mcp --connect` taking a bare loopback port while refusing any other host.

use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_localphoto-cli")).args(args).output().unwrap()
}

fn dir(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("localphoto-cli-mould-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

#[test]
fn commands_search_filters_the_catalogue() {
    let all = cli(&["commands", "--json"]);
    assert!(all.status.success(), "{}", String::from_utf8_lossy(&all.stderr));
    let all: Vec<serde_json::Value> = serde_json::from_slice(&all.stdout).unwrap();
    let out = cli(&["commands", "--json", "--search", "develop"]);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let found: Vec<serde_json::Value> = serde_json::from_slice(&out.stdout).unwrap();
    assert!(!found.is_empty() && found.len() < all.len(), "{} of {}", found.len(), all.len());
    for c in &found {
        let matched = ["id", "label"].iter().any(|k| c[*k].as_str().is_some_and(|v| v.to_lowercase().contains("develop")));
        assert!(matched, "{} does not match `develop`", c["id"]);
    }
}

#[test]
fn a_script_may_be_a_json_array() {
    let d = dir("array");
    let script = d.join("steps.json");
    std::fs::write(&script, r#"["develop.controls"]"#).unwrap();
    let out = cli(&["run", "--demo", "--script", script.to_str().unwrap()]);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let printed = String::from_utf8(out.stdout).unwrap();
    assert_eq!(printed.lines().count(), 1, "{printed}");
    let reply: serde_json::Value = serde_json::from_str(printed.lines().next().unwrap_or_default()).unwrap();
    assert_eq!(reply["ok"], true, "{reply}");
}

#[test]
fn a_bad_array_step_names_its_index() {
    let d = dir("bad");
    let script = d.join("steps.json");
    std::fs::write(&script, r#"["develop.controls", {"params": {}}]"#).unwrap();
    let out = cli(&["run", "--demo", "--script", script.to_str().unwrap()]);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("[1]") && stderr.contains("needs `command` or `method`"), "{stderr}");
}

#[test]
fn connect_takes_a_bare_port_and_refuses_a_public_host() {
    // A bare port means loopback: the lazy bridge names 127.0.0.1:1 and serves until stdin closes.
    let out = Command::new(env!("CARGO_BIN_EXE_localphoto-cli")).args(["mcp", "--connect", "1"]).stdin(Stdio::null()).output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(String::from_utf8_lossy(&out.stderr).contains("127.0.0.1:1"), "{}", String::from_utf8_lossy(&out.stderr));
    let out =
        Command::new(env!("CARGO_BIN_EXE_localphoto-cli")).args(["mcp", "--connect", "192.168.1.5:7980"]).stdin(Stdio::null()).output().unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("loopback only"));
}
