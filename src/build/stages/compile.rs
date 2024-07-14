use std::io::Error;
use std::process::Command;

use crate::log::system_msg_webhook;
use crate::log::{
    message_template_build_failed, message_template_build_succeeded,
    message_template_starting_build,
};

/// Runs the cargo build --release command for the project
pub fn run_build(repo_name: &str) -> Result<String, Error> {
    println!("Running cargo build --release...");
    let command = format!("cd ~/watchdog_rs/~/deploys/{} && cargo build --release", repo_name);
    let copy_binary_command = format!(
        "cp ~/watchdog_rs/~/deploys/{}/target/release/{} ~/watchdog_rs/~/deploys/{}/{}",
        repo_name, repo_name, repo_name,repo_name
    );
    println!("Executing command: {}", command);

    let output = Command::new("sh").arg("-c").arg(&command).output()?;
    let copy_output = Command::new("sh").arg("-c").arg(&copy_binary_command).output()?;

    println!("Command executed. Processing output...");

    let stdout_output = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr_output = String::from_utf8_lossy(&output.stderr).to_string();

    let stdout_copy_output = String::from_utf8_lossy(&copy_output.stdout).to_string();
    let stderr_copy_output = String::from_utf8_lossy(&copy_output.stderr).to_string();

    println!("Copy Output: {}", stdout_copy_output);
    println!("Copy Error: {}", stderr_copy_output);

    println!("Standard Output: {}", stdout_output);
    println!("Standard Error: {}", stderr_output);

    std::thread::sleep(std::time::Duration::from_secs(12));
    let binary_path: String = format!("~/watchdog_rs/~/deploys/{}/{}", repo_name, repo_name);
    let binary_file = shellexpand::tilde(&binary_path).to_string();
    if std::path::Path::new(&binary_file).exists() {
        println!("Build succeeded");
    } else {
        println!("Build failed");
        return Err(Error::new(std::io::ErrorKind::Other, "Build failed"));
    }

    println!("Build succeeded");
    Ok(stdout_output)
}

pub async fn rust_build(
    build_id: &str,
    repo_name: &str,
    repo_url: &str,
    webhook_url: &str,
) -> Result<(), Error> {
    println!("Starting build...");

    let message: String = message_template_starting_build(build_id, repo_url, repo_name);
    println!("Generated starting build message: {}", message);

    match system_msg_webhook(&message, webhook_url).await {
        Ok(_) => println!("Successfully sent starting build message."),
        Err(e) => {
            eprintln!("Failed to send system message: {}", e);
        }
    }

    println!("Running build for repository: {}", repo_name);
    match run_build(repo_name) {
        Ok(build_output) => {
            println!("Build succeeded. Generating success message...");
            let message: String = message_template_build_succeeded(build_id, repo_url, repo_name);
            println!("Generated build successful message: {}", message);

            match system_msg_webhook(&message, webhook_url).await {
                Ok(_) => println!("Successfully sent build successful message."),
                Err(e) => {
                    eprintln!("Failed to send system message: {}", e);
                }
            }
            println!("Build completed successfully.");
            Ok(())
        }
        Err(e) => {
            println!("Build failed. Generating failure message...");
            let message: String = message_template_build_failed(build_id, repo_url, repo_name);
            println!("Generated build failed message: {}", message);

            match system_msg_webhook(&message, webhook_url).await {
                Ok(_) => println!("Successfully sent build failed message."),
                Err(e) => {
                    eprintln!("Failed to send system message: {}", e);
                }
            }
            eprintln!("Build failed: {}", e);
            Err(e)
        }
    }
}
