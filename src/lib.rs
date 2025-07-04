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

pub fn is_nvm_installed() -> bool {
    Command::new("/bin/bash")
        .arg("-c")
        .arg("source $HOME/.nvm/nvm.sh && nvm --version")
        .output()
        .is_ok()
}

pub fn install_nvm() {
    println!("Installing nvm...");
    let output = Command::new("/bin/bash")
        .arg("-c")
        .arg("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("nvm installed successfully.");
                println!("Installing latest LTS Node.js...");
                let lts_output = Command::new("/bin/bash")
                    .arg("-c")
                    .arg("source $HOME/.nvm/nvm.sh && nvm install --lts")
                    .output();
                match lts_output {
                    Ok(lts_output) => {
                        if lts_output.status.success() {
                            println!("Latest LTS Node.js installed successfully.");
                        } else {
                            eprintln!("Failed to install LTS Node.js: {}", String::from_utf8_lossy(&lts_output.stderr));
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to execute nvm command: {}", e);
                    }
                }
            } else {
                eprintln!("Failed to install nvm: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}

pub fn is_vscode_installed() -> bool {
    Command::new("code")
        .arg("--version")
        .output()
        .is_ok()
}

pub fn install_vscode() {
    println!("Installing Visual Studio Code with Homebrew...");
    let output = Command::new("brew")
        .arg("install")
        .arg("--cask")
        .arg("visual-studio-code")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Visual Studio Code installed successfully.");
            } else {
                eprintln!("Failed to install Visual Studio Code: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}
