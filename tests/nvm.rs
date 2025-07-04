use mac_setup_cli::{is_nvm_installed, install_nvm};
use std::process::Command;

#[test]
fn test_is_nvm_installed() {
    // This test is dependent on the environment.
    // If nvm is installed, this should pass.
    assert!(is_nvm_installed());
}

#[test]
fn test_install_nvm_and_lts_node() {
    // This is a tricky test to write without actually installing nvm.
    // We will simulate the installation by checking if the command can be executed.
    if !is_nvm_installed() {
        install_nvm();
        assert!(is_node_lts());
    } else {
        println!("Skipping nvm installation test because nvm is already installed.");
    }
}

#[test]
fn test_nvm_version() {
    if is_nvm_installed() {
        let output = Command::new("/bin/bash")
            .arg("-c")
            .arg("source $HOME/.nvm/nvm.sh && nvm --version")
            .output()
            .expect("Failed to execute command");
        assert!(output.status.success());
        let version = String::from_utf8_lossy(&output.stdout);
        assert!(!version.is_empty());
    }
}

#[test]
fn test_is_node_lts() {
    if is_nvm_installed() {
        assert!(is_node_lts());
    }
}

fn is_node_lts() -> bool {
    let output = Command::new("/bin/bash")
        .arg("-c")
        .arg("source $HOME/.nvm/nvm.sh && nvm ls --no-colors")
        .output()
        .expect("Failed to execute command");
    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("lts")
}

