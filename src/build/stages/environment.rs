use std::io::Error;
use std::process::Command;

use crate::log::system_msg_webhook;
use crate::log::{message_template_setting_up_env, message_template_copying_repository};


/// Navigates to the deploy directory
pub fn go_to_deploy_directory() -> Result<(), Error> {
    // Create the deploy directory and navigate to it
    Command::new("mkdir").args(&["-p", "~/deploys"]).status()?;
    Command::new("cd").arg("~/deploys").status()?;

    Ok(())
}


/// delete old deploy repo if it exists
pub fn delete_old_repo(repo_name: &str) -> Result<(), Error> {
    let repo_path: String = format!("~/deploys/{}", repo_name);
    Command::new("rm").args(&["-rf", &repo_path]).status()?;
    Ok(())
}


/// Clones the repository
pub fn clone_repo(repo_url: &str, repo_name: &str) -> Result<(), Error> {
    Command::new("git")
        .args(&["clone", "--depth", "1", repo_url, &format!("./{}", repo_name)])
        .status()?;
    Ok(())
}

/// Navigates to the repository directory
pub fn go_to_repo_directory(repo_name: &str) -> Result<(), Error> {
    Command::new("cd").arg(&format!("./{}", repo_name)).status()?;
    Ok(())
}



pub fn rust_environment(
    repo_name: &str,
    repo_url: &str
) -> Result<(), Error> {
    // Create the deploy directory and navigate to it
    go_to_deploy_directory()?;

    // Delete the old repository if it exists
    delete_old_repo(repo_name)?;

    // Clone the repository
    clone_repo(repo_url, repo_name)?;

    // Navigate to the repository directory
    go_to_repo_directory(repo_name)?;

    Ok(())
}