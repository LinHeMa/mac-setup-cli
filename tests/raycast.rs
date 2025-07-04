use my_rust_cli::{install_raycast, is_homebrew_installed, is_raycast_installed, open_raycast};
use std::process::Command;

#[test]
fn test_is_homebrew_installed_for_raycast() {
    assert!(is_homebrew_installed());
}

#[test]
fn test_is_raycast_installed() {
    assert!(is_raycast_installed());
}

#[test]
fn test_install_raycast() {
    if is_homebrew_installed() {
        if !is_raycast_installed() {
            install_raycast();
            assert!(is_raycast_installed());
        } else {
            println!("Skipping Raycast installation test because Raycast is already installed.");
        }
    } else {
        println!("Skipping Raycast installation test because Homebrew is not installed.");
    }
}

#[test]
fn test_raycast_version() {
    if is_raycast_installed() {
        // Raycast does not have a command line version flag, so we check the app's Info.plist
        let output = Command::new("defaults")
            .arg("read")
            .arg("/Applications/Raycast.app/Contents/Info.plist")
            .arg("CFBundleShortVersionString")
            .output()
            .expect("Failed to execute command");
        assert!(output.status.success());
        let version = String::from_utf8_lossy(&output.stdout);
        assert!(!version.trim().is_empty());
    }
}

#[test]
fn test_open_raycast() {
    if is_raycast_installed() {
        open_raycast();
    }
}
