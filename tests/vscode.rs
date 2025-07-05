#![cfg(feature = "gui-tests")]

use mac_setup_cli::{is_vscode_installed, is_homebrew_installed, install_vscode, open_vscode};
use std::process::Command;

#[test]
fn test_is_homebrew_installed_for_vscode() {
    // This test is dependent on the environment.
    // If Homebrew is installed, this should pass.
    assert!(is_homebrew_installed());
}

#[test]
fn test_is_vscode_installed() {
    // This test is dependent on the environment.
    // If VS Code is installed, this should pass.
    assert!(is_vscode_installed());
}

#[test]
fn test_install_vscode() {
    // This is a tricky test to write without actually installing VS Code.
    // We will simulate the installation by checking if the command can be executed.
    if is_homebrew_installed() {
        if !is_vscode_installed() {
            install_vscode();
            assert!(is_vscode_installed());
        } else {
            println!("Skipping VS Code installation test because VS Code is already installed.");
        }
    } else {
        println!("Skipping VS Code installation test because Homebrew is not installed.");
    }
}

#[test]
fn test_vscode_version() {
    if is_vscode_installed() {
        let output = Command::new("code")
            .arg("--version")
            .output()
            .expect("Failed to execute command");
        assert!(output.status.success());
        let version = String::from_utf8_lossy(&output.stdout);
        assert!(!version.is_empty());
    }
}

#[test]
fn test_open_vscode() {
    if is_vscode_installed() {
        open_vscode();
    }
}
