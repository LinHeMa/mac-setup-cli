#![cfg(feature = "docker-tests")]

use mac_setup_cli::{is_docker_installed, is_homebrew_installed, install_docker, open_docker};
use std::process::Command;

#[test]
fn test_is_homebrew_installed_for_docker() {
    assert!(is_homebrew_installed());
}

#[test]
fn test_is_docker_installed() {
    assert!(is_docker_installed());
}

#[test]
fn test_install_docker() {
    if is_homebrew_installed() {
        if !is_docker_installed() {
            install_docker();
            assert!(is_docker_installed());
        } else {
            println!("Skipping Docker installation test because Docker is already installed.");
        }
    } else {
        println!("Skipping Docker installation test because Homebrew is not installed.");
    }
}

#[test]
fn test_docker_version() {
    if is_docker_installed() {
        let output = Command::new("docker")
            .arg("--version")
            .output()
            .expect("Failed to execute command");
        assert!(output.status.success());
        let version = String::from_utf8_lossy(&output.stdout);
        assert!(!version.is_empty());
    }
}

#[test]
fn test_open_docker() {
    if is_docker_installed() {
        open_docker();
    }
}
