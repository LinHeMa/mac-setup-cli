use std::process::Command;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    loop {
        let selections = &["Check for git", "Exit"];

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
                if is_git_installed() {
                    println!("Git is installed.");
                } else {
                    println!("Git is not installed.");
                }
                println!("\n(Returning to menu in 2 seconds)");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            1 => {
                println!("Bye!");
                break;
            }
            _ => unreachable!(),
        }
    }
}

fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .is_ok()
}

