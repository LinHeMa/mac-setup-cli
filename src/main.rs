use dialoguer::{theme::ColorfulTheme, Select, Confirm};
use mac_setup_cli::{is_git_installed, install_git_with_homebrew, is_homebrew_installed, install_homebrew, is_nvm_installed, install_nvm, is_vscode_installed, install_vscode, is_chrome_installed, install_chrome, is_docker_installed, install_docker, is_slack_installed, install_slack, is_raycast_installed, install_raycast};

fn main() {
    loop {
        let selections = &["Check for git", "Check for Homebrew", "Check for nvm", "Check for VS Code", "Check for Google Chrome", "Check for Docker", "Check for Slack", "Check for Raycast", "Exit"];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .default(0)
            .items(&selections[..])
            .interact_opt()
        {
            Ok(Some(selection)) => selection,
            Ok(None) => {
                println!("No selection made or non-interactive terminal. Exiting.");
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        };

        match selection {
            0 => {
                if !is_homebrew_installed() {
                    println!("Please install Homebrew first.");
                } else if is_git_installed() {
                    println!("Git is installed.");
                } else {
                    println!("Git is not installed.");
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Do you want to install Git using Homebrew?")
                        .interact()
                        .unwrap_or(false)
                    {
                        install_git_with_homebrew();
                    }
                }
                println!("\n(Returning to menu in 2 seconds)");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            1 => {
                if is_homebrew_installed() {
                    println!("Homebrew is installed.");
                } else {
                    println!("Homebrew is not installed.");
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Do you want to install Homebrew?")
                        .interact()
                        .unwrap_or(false)
                    {
                        install_homebrew();
                    }
                }
                println!("\n(Returning to menu in 2 seconds)");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            2 => {
                if is_nvm_installed() {
                    println!("nvm is installed.");
                } else {
                    println!("nvm is not installed.");
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Do you want to install nvm?")
                        .interact()
                        .unwrap_or(false)
                    {
                        install_nvm();
                    }
                }
                println!("\n(Returning to menu in 2 seconds)");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            3 => {
                if !is_homebrew_installed() {
                    println!("Please install Homebrew first.");
                } else if is_vscode_installed() {
                    println!("Visual Studio Code is installed.");
                } else {
                    println!("Visual Studio Code is not installed.");
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Do you want to install Visual Studio Code using Homebrew?")
                        .interact()
                        .unwrap_or(false)
                    {
                        install_vscode();
                    }
                }
                println!("\n(Returning to menu in 2 seconds)");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            4 => {
                if !is_homebrew_installed() {
                    println!("Please install Homebrew first.");
                } else if is_chrome_installed() {
                    println!("Google Chrome is installed.");
                } else {
                    println!("Google Chrome is not installed.");
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Do you want to install Google Chrome using Homebrew?")
                        .interact()
                        .unwrap_or(false)
                    {
                        install_chrome();
                    }
                }
                println!("\n(Returning to menu in 2 seconds)");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            5 => {
                if !is_homebrew_installed() {
                    println!("Please install Homebrew first.");
                } else if is_docker_installed() {
                    println!("Docker is installed.");
                } else {
                    println!("Docker is not installed.");
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Do you want to install Docker using Homebrew?")
                        .interact()
                        .unwrap_or(false)
                    {
                        install_docker();
                    }
                }
                println!("\n(Returning to menu in 2 seconds)");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            6 => {
                if !is_homebrew_installed() {
                    println!("Please install Homebrew first.");
                } else if is_slack_installed() {
                    println!("Slack is installed.");
                } else {
                    println!("Slack is not installed.");
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Do you want to install Slack using Homebrew?")
                        .interact()
                        .unwrap_or(false)
                    {
                        install_slack();
                    }
                }
                println!("\n(Returning to menu in 2 seconds)");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            7 => {
                if !is_homebrew_installed() {
                    println!("Please install Homebrew first.");
                } else if is_raycast_installed() {
                    println!("Raycast is installed.");
                } else {
                    println!("Raycast is not installed.");
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Do you want to install Raycast using Homebrew?")
                        .interact()
                        .unwrap_or(false)
                    {
                        install_raycast();
                    }
                }
                println!("\n(Returning to menu in 2 seconds)");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            8 => {
                println!("Bye!");
                break;
            }
            _ => unreachable!(),
        }
    }
}



