use std::process::Command;

pub fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .is_ok()
}

pub fn install_git_with_homebrew() {
    println!("Installing Git with Homebrew...");
    let output = Command::new("brew")
        .arg("install")
        .arg("git")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Git installed successfully.");
            } else {
                eprintln!("Failed to install Git: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}

pub fn is_homebrew_installed() -> bool {
    Command::new("brew")
        .arg("--version")
        .output()
        .is_ok()
}

pub fn install_homebrew() {
    println!("Installing Homebrew...");
    let output = Command::new("/bin/bash")
        .arg("-c")
        .arg("$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Homebrew installed successfully.");
            } else {
                eprintln!("Failed to install Homebrew: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}
