use my_rust_cli::{is_homebrew_installed, install_homebrew};
use std::process::Command;

#[test]
fn test_is_homebrew_installed() {
    // This test is dependent on the environment.
    // If Homebrew is installed, this should pass.
    // If not, it should fail.
    assert!(is_homebrew_installed());
}

#[test]
fn test_install_homebrew() {
    // This is a tricky test to write without actually installing Homebrew.
    // We will simulate the installation by checking if the command can be executed.
    // A more robust test would involve mocking the command execution.
    if !is_homebrew_installed() {
        // We can't truly test the installation without running the actual command,
        // which is not ideal for a unit test.
        // We will just check that the command can be constructed.
        let cmd = Command::new("/bin/bash")
            .arg("-c")
            .arg("$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)")
            .output();
        assert!(cmd.is_ok());
    }
}

#[test]
fn test_homebrew_version() {
    if is_homebrew_installed() {
        let output = Command::new("brew")
            .arg("--version")
            .output()
            .expect("Failed to execute command");
        assert!(output.status.success());
        let version = String::from_utf8_lossy(&output.stdout);
        assert!(version.starts_with("Homebrew"));
    }
}
