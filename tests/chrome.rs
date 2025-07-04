use mac_setup_cli::{is_chrome_installed, is_homebrew_installed, install_chrome, open_chrome};
use std::process::Command;

#[test]
fn test_is_homebrew_installed_for_chrome() {
    // This test is dependent on the environment.
    // If Homebrew is installed, this should pass.
    assert!(is_homebrew_installed());
}

#[test]
fn test_is_chrome_installed() {
    // This test is dependent on the environment.
    // If Chrome is installed, this should pass.
    assert!(is_chrome_installed());
}

#[test]
fn test_install_chrome() {
    // This is a tricky test to write without actually installing Chrome.
    // We will simulate the installation by checking if the command can be executed.
    if is_homebrew_installed() {
        if !is_chrome_installed() {
            install_chrome();
            assert!(is_chrome_installed());
        } else {
            println!("Skipping Chrome installation test because Chrome is already installed.");
        }
    } else {
        println!("Skipping Chrome installation test because Homebrew is not installed.");
    }
}

#[test]
fn test_chrome_version() {
    if is_chrome_installed() {
        let output = Command::new("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
            .arg("--version")
            .output()
            .expect("Failed to execute command");
        assert!(output.status.success());
        let version = String::from_utf8_lossy(&output.stdout);
        assert!(!version.is_empty());
    }
}

#[test]
fn test_open_chrome() {
    if is_chrome_installed() {
        open_chrome();
    }
}
