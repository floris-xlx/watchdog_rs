use std::io::Error;
use std::process::Command;

use crate::log::system_msg_webhook;
use crate::log::{message_template_copying_repository, message_template_setting_up_env};

/// Navigates to the deploy directory
pub fn go_to_deploy_directory() -> Result<(), Error> {
    use std::env;
    use std::fs;
    use std::path::Path;

    println!("Creating and navigating to the deploy directory...");

    Command::new("sh")
        .arg("-c")
        .arg("cd ~ && mkdir -p deploys")
        .status()?;

    println!("Successfully navigated to the deploy directory.");
    Ok(())
}

/// delete old deploy repo if it exists
pub fn delete_old_repo(repo_name: &str) -> Result<(), Error> {
    println!("Deleting old repository if it exists...");
    let repo_path: String = format!("deploys/{}", repo_name);
    Command::new("rm").args(&["-rf", &repo_path]).status()?;
    println!("Old repository deleted if it existed.");
    Ok(())
}

/// Clones the repository
pub fn clone_repo(repo_url: &str, repo_name: &str) -> Result<(), Error> {
    println!("Cloning the repository...");
    Command::new("git")
        .args(&[
            "clone",
            "--depth",
            "1",
            repo_url,
            &format!("deploys/{}", repo_name),
        ])
        .status()?;

    // print full path of the repository
    Command::new("ls").status()?;

    println!("Repository cloned successfully.");
    Ok(())
}

pub async fn rust_environment(
    build_id: &str,
    repo_name: &str,
    repo_url: &str,
    webhook_url: &str,
) -> Result<(), Error> {
    println!("Setting up the Rust environment...");

    let message: String = message_template_setting_up_env(build_id, repo_url, repo_name);

    match system_msg_webhook(&message, webhook_url).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to send system message: {}", e);
        }
    }

    // Create the deploy directory and navigate to it
    go_to_deploy_directory()?;

    // Delete the old repository if it exists
    delete_old_repo(repo_name)?;



    // clone msg template
    let message: String = message_template_copying_repository(build_id, repo_url, repo_name);

    match system_msg_webhook(&message, webhook_url).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to send system message: {}", e);
        }
    }

    // Clone the repository
    clone_repo(repo_url, repo_name)?;

    println!("Rust environment setup completed successfully.");
    Ok(())
}
