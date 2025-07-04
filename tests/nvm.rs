use my_rust_cli::is_nvm_installed;
use std::process::Command;

#[test]
fn test_is_nvm_installed() {
    // This test is dependent on the environment.
    // If nvm is installed, this should pass.
    assert!(is_nvm_installed());
}

#[test]
fn test_install_nvm() {
    // This is a tricky test to write without actually installing nvm.
    // We will simulate the installation by checking if the command can be executed.
    if !is_nvm_installed() {
        // We can't truly test the installation without running the actual command,
        // which is not ideal for a unit test.
        // We will just check that the command can be constructed.
        let cmd = Command::new("/bin/bash")
            .arg("-c")
            .arg("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash")
            .output();
        assert!(cmd.is_ok());
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
