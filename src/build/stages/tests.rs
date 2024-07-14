use std::io::Error;
use std::process::Command;

use crate::log::system_msg_webhook;
use crate::log::{message_template_starting_tests, message_template_tests_passed, message_template_tests_failed};

/// Runs the tests for the project
pub fn run_tests(repo_name: &str) -> Result<String, Error> {
    println!("Running tests...");
    // only when running cargo r and not as standalone binary
    // then ~/watchdog_rs/~/ will b replaced with ~/ in the command
    
    let command = format!("cd ~/watchdog_rs/~/deploys/{} && cargo test && ls", repo_name);
    println!("Executing command: {}", command);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()?;

    println!("Command executed. Processing output...");

    let stdout_output = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr_output = String::from_utf8_lossy(&output.stderr).to_string();

    println!("Standard Output: {}", stdout_output);
    println!("Standard Error: {}", stderr_output);

    if !stdout_output.contains("test result: ok") {
        println!("Tests failed");
        return Err(Error::new(std::io::ErrorKind::Other, "Tests failed"));
    }

    println!("Tests passed");
    Ok(stdout_output)
}

pub async fn rust_tests(
    build_id: &str,
    repo_name: &str,
    repo_url: &str,
    webhook_url: &str,
) -> Result<(), Error> {
    println!("Starting tests...");

    let message: String = message_template_starting_tests(build_id, repo_url, repo_name);
    println!("Generated starting tests message: {}", message);

    match system_msg_webhook(&message, webhook_url).await {
        Ok(_) => println!("Successfully sent starting tests message."),
        Err(e) => {
            eprintln!("Failed to send system message: {}", e);
        }
    }

    println!("Running tests for repository: {}", repo_name);
    match run_tests(repo_name) {
        Ok(test_output) => {
            println!("Tests passed. Generating success message...");
            let message: String = message_template_tests_passed(build_id, repo_url, repo_name);
            println!("Generated tests passed message: {}", message);

            match system_msg_webhook(&message, webhook_url).await {
                Ok(_) => println!("Successfully sent tests passed message."),
                Err(e) => {
                    eprintln!("Failed to send system message: {}", e);
                }
            }
            println!("Tests completed successfully.");
            Ok(())
        }
        Err(e) => {
            println!("Tests failed. Generating failure message...");
            let message: String = message_template_tests_failed(build_id, repo_url, repo_name);
            println!("Generated tests failed message: {}", message);

            match system_msg_webhook(&message, webhook_url).await {
                Ok(_) => println!("Successfully sent tests failed message."),
                Err(e) => {
                    eprintln!("Failed to send system message: {}", e);
                }
            }
            eprintln!("Tests failed: {}", e);
            Err(e)
        }
    }
}
