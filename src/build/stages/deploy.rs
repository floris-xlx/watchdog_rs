use std::io::Error;
use std::process::Command;

use crate::log::system_msg_webhook;
use crate::log::{
    message_template_trying_to_deploy, message_template_failed_to_deploy,
    message_template_successfully_deployed,
};

/// Runs the deployment script for the project
pub fn run_deploy(service_name: &str) -> Result<String, Error> {
    println!("Running deployment script...");

    // Stop the old service
    let stop_command = format!("pkill -f ./{}", service_name);
    println!("Executing command: {}", stop_command);
    let stop_output = Command::new("sh").arg("-c").arg(&stop_command).output()?;
    println!("Stop command executed. Processing output...");

    let stdout_stop_output = String::from_utf8_lossy(&stop_output.stdout).to_string();
    let stderr_stop_output = String::from_utf8_lossy(&stop_output.stderr).to_string();
    println!("Stop Standard Output: {}", stdout_stop_output);
    println!("Stop Standard Error: {}", stderr_stop_output);

    // Start the service
    let start_command = format!("nohup deploys/{}/services/xylex_api/xylex_api &", service_name);
    println!("Executing command: {}", start_command);
    let start_output = Command::new("sh").arg("-c").arg(&start_command).output()?;
    println!("Start command executed. Processing output...");

    let stdout_start_output = String::from_utf8_lossy(&start_output.stdout).to_string();
    let stderr_start_output = String::from_utf8_lossy(&start_output.stderr).to_string();
    println!("Start Standard Output: {}", stdout_start_output);
    println!("Start Standard Error: {}", stderr_start_output);

    // Check if the service is running
    let check_command = format!("pgrep -f ./{}", service_name);
    println!("Executing command: {}", check_command);
    let check_output = Command::new("sh").arg("-c").arg(&check_command).output()?;
    println!("Check command executed. Processing output...");

    let stdout_check_output = String::from_utf8_lossy(&check_output.stdout).to_string();
    let stderr_check_output = String::from_utf8_lossy(&check_output.stderr).to_string();
    println!("Check Standard Output: {}", stdout_check_output);
    println!("Check Standard Error: {}", stderr_check_output);

    if stdout_check_output.is_empty() {
        println!("Service failed to start");
        return Err(Error::new(std::io::ErrorKind::Other, "Service failed to start"));
    }

    println!("Deployment succeeded");
    
    Ok(stdout_check_output)
}

pub async fn rust_deploy(
    build_id: &str,
    service_name: &str,
    repo_url: &str,
    webhook_url: &str,
) -> Result<(), Error> {
    println!("Starting deployment...");

    let message: String = message_template_trying_to_deploy(build_id, repo_url, service_name);
    println!("Generated starting deployment message: {}", message);

    match system_msg_webhook(&message, webhook_url).await {
        Ok(_) => println!("Successfully sent starting deployment message."),
        Err(e) => {
            eprintln!("Failed to send system message: {}", e);
        }
    }

    println!("Running deployment for service: {}", service_name);
    match run_deploy(service_name) {
        Ok(deploy_output) => {
            println!("Deployment succeeded. Generating success message...");
            let message: String = message_template_successfully_deployed(build_id, repo_url, service_name);
            println!("Generated deployment successful message: {}", message);

            match system_msg_webhook(&message, webhook_url).await {
                Ok(_) => println!("Successfully sent deployment successful message."),
                Err(e) => {
                    eprintln!("Failed to send system message: {}", e);
                }
            }
            println!("Deployment completed successfully.");
            Ok(())
        }
        Err(e) => {
            println!("Deployment failed. Generating failure message...");
            let message: String = message_template_failed_to_deploy(build_id, repo_url, service_name);
            println!("Generated deployment failed message: {}", message);

            match system_msg_webhook(&message, webhook_url).await {
                Ok(_) => println!("Successfully sent deployment failed message."),
                Err(e) => {
                    eprintln!("Failed to send system message: {}", e);
                }
            }
            eprintln!("Deployment failed: {}", e);
            Err(e)
        }
    }
}
