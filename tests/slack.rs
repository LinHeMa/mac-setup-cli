#![cfg(feature = "gui-tests")]

use mac_setup_cli::{is_slack_installed, is_homebrew_installed, install_slack, open_slack};
use std::process::Command;

#[test]
fn test_is_homebrew_installed_for_slack() {
    assert!(is_homebrew_installed());
}

#[test]
fn test_is_slack_installed() {
    assert!(is_slack_installed());
}

#[test]
fn test_install_slack() {
    if is_homebrew_installed() {
        if !is_slack_installed() {
            install_slack();
            assert!(is_slack_installed());
        } else {
            println!("Skipping Slack installation test because Slack is already installed.");
        }
    } else {
        println!("Skipping Slack installation test because Homebrew is not installed.");
    }
}

#[test]
fn test_slack_version() {
    if is_slack_installed() {
        let output = Command::new("defaults")
            .arg("read")
            .arg("/Applications/Slack.app/Contents/Info.plist")
            .arg("CFBundleShortVersionString")
            .output()
            .expect("Failed to execute command");
        assert!(output.status.success());
        let version = String::from_utf8_lossy(&output.stdout);
        assert!(!version.trim().is_empty());
    }
}

#[test]
fn test_open_slack() {
    if is_slack_installed() {
        open_slack();
    }
}
