use std::path::Path;
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
            eprintln!("Failed to execute command: {e}");
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
            eprintln!("Failed to execute command: {e}");
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
                        eprintln!("Failed to execute nvm command: {e}");
                    }
                }
            } else {
                eprintln!("Failed to install nvm: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {e}");
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
                open_vscode();
            } else {
                eprintln!("Failed to install Visual Studio Code: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {e}");
        }
    }
}

pub fn open_vscode() {
    Command::new("code")
        .arg(".")
        .output()
        .expect("Failed to open VS Code");
}

pub fn is_chrome_installed() -> bool {
    Path::new("/Applications/Google Chrome.app").exists()
}

pub fn install_chrome() {
    println!("Installing Google Chrome with Homebrew...");
    let output = Command::new("brew")
        .arg("install")
        .arg("--cask")
        .arg("google-chrome")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Google Chrome installed successfully.");
                open_chrome();
            } else {
                eprintln!("Failed to install Google Chrome: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {e}");
        }
    }
}

pub fn open_chrome() {
    Command::new("open")
        .arg("-a")
        .arg("Google Chrome")
        .output()
        .expect("Failed to open Google Chrome");
}

pub fn is_docker_installed() -> bool {
    Command::new("docker")
        .arg("--version")
        .output()
        .is_ok()
}

pub fn install_docker() {
    println!("Installing Docker with Homebrew...");
    let output = Command::new("brew")
        .arg("install")
        .arg("--cask")
        .arg("docker")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Docker installed successfully.");
                open_docker();
            } else {
                eprintln!("Failed to install Docker: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {e}");
        }
    }
}

pub fn open_docker() {
    Command::new("open")
        .arg("-a")
        .arg("Docker")
        .output()
        .expect("Failed to open Docker");
}

pub fn is_slack_installed() -> bool {
    Path::new("/Applications/Slack.app").exists()
}

pub fn install_slack() {
    println!("Installing Slack with Homebrew...");
    let output = Command::new("brew")
        .arg("install")
        .arg("--cask")
        .arg("slack")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Slack installed successfully.");
                open_slack();
            } else {
                eprintln!("Failed to install Slack: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {e}");
        }
    }
}

pub fn open_slack() {
    Command::new("open")
        .arg("-a")
        .arg("Slack")
        .output()
        .expect("Failed to open Slack");
}

pub fn is_raycast_installed() -> bool {
    Path::new("/Applications/Raycast.app").exists()
}

pub fn install_raycast() {
    println!("Installing Raycast with Homebrew...");
    let output = Command::new("brew")
        .arg("install")
        .arg("--cask")
        .arg("raycast")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Raycast installed successfully.");
                open_raycast();
            } else {
                eprintln!("Failed to install Raycast: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {e}");
        }
    }
}

pub fn open_raycast() {
    Command::new("open")
        .arg("-a")
        .arg("Raycast")
        .output()
        .expect("Failed to open Raycast");
}
