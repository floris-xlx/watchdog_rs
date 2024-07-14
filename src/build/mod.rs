pub mod rust;
pub mod stages;

use std::error::Error;
use std::io::Error as IOError;

use crate::api::client::build;
// crate imports
use crate::build::stages::environment;
use crate::log::{
    discord_log_webhook, message_template_build_failed, message_template_schedule_build,
    system_msg_webhook,
};

pub async fn schedule_build(
    repository_url: &str,
    build_id: &str,
    service_name: &str,
    webhook_url: &str,
    repository_with_key: &str,
) -> Result<(), Box<dyn Error>> {
    let message: String = message_template_schedule_build(build_id, repository_url, service_name);

    match system_msg_webhook(&message, webhook_url).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to send system message: {}", e);
        }
    }

    let build_failed_msg = message_template_build_failed(
        build_id,
        repository_url,
        service_name,
    );

    let result: Result<(), IOError> =
        environment::rust_environment(service_name, repository_with_key);

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        let message: String = format!("Failed to setup environment for {}", service_name);

        let log_result: Result<(), Box<dyn Error>> =
            discord_log_webhook(&message, webhook_url).await;

        if let Err(e) = system_msg_webhook(&build_failed_msg, webhook_url).await {
            eprintln!("Failed to send build failed message: {}", e);
        }

        if log_result.is_err() {
            eprintln!("Failed to send log message to Discord");
        }

        return Err(Box::new(e));
    }

    Ok(())
}
