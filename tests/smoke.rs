use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_ytmp3")
}

#[test]
fn version_flag_prints_crate_version() {
    let out = Command::new(bin()).arg("--version").output().unwrap();
    assert!(out.status.success(), "--version failed: {:?}", out);
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains(env!("CARGO_PKG_VERSION")), "stdout: {s}");
}

#[test]
fn help_lists_new_flags() {
    let out = Command::new(bin()).arg("--help").output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    for flag in ["--bitrate", "--format", "--output-dir", "--quiet"] {
        assert!(s.contains(flag), "--help missing {flag}\nhelp:\n{s}");
    }
}

#[test]
fn missing_url_errors() {
    let out = Command::new(bin()).output().unwrap();
    assert!(!out.status.success());
}
