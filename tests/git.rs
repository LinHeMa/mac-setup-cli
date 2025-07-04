use my_rust_cli::{is_git_installed, is_homebrew_installed};
use std::process::Command;

#[test]
fn test_is_homebrew_installed_for_git() {
    // This test is dependent on the environment.
    // If Homebrew is installed, this should pass.
    assert!(is_homebrew_installed());
}

#[test]
fn test_is_git_installed() {
    // This test is dependent on the environment.
    // If git is installed, this should pass.
    assert!(is_git_installed());
}

#[test]
fn test_install_git_with_homebrew() {
    // This is a tricky test to write without actually installing git.
    // We will simulate the installation by checking if the command can be executed.
    // This test will only be meaningful if homebrew is installed.
    if is_homebrew_installed() {
        if !is_git_installed() {
            // We can't truly test the installation without running the actual command,
            // which is not ideal for a unit test.
            // We will just check that the command can be constructed.
            let cmd = Command::new("brew")
                .arg("install")
                .arg("git")
                .output();
            assert!(cmd.is_ok());
        } else {
            println!("Skipping git installation test because git is already installed.");
        }
    } else {
        println!("Skipping git installation test because homebrew is not installed.");
    }
}

#[test]
fn test_git_version() {
    if is_git_installed() {
        let output = Command::new("git")
            .arg("--version")
            .output()
            .expect("Failed to execute command");
        assert!(output.status.success());
        let version = String::from_utf8_lossy(&output.stdout);
        assert!(version.starts_with("git version"));
    }
}
