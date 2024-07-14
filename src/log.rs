use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub async fn discord_log_webhook(
    log_message: &str,
    webhook_url: &str,
) -> Result<(), Box<dyn Error>> {
    let client: Client = Client::new();

    let formatted_log_message: String = format!("```watchdog_rs | {}```", log_message);

    println!("\x1b[34;1m{}\x1b[0m", formatted_log_message);

    client
        .post(webhook_url)
        .body(
            json!({
                "content": formatted_log_message
            })
            .to_string(),
        )
        .header("Content-Type", "application/json")
        .send()
        .await?;

    Ok(())
}

pub async fn system_msg_webhook(message: &str, webhook_url: &str) -> Result<(), Box<dyn Error>> {
    let client: Client = Client::new();

    client
        .post(webhook_url)
        .body(
            json!({
                "content": message
            })
            .to_string(),
        )
        .header("Content-Type", "application/json")
        .send()
        .await?;

    Ok(())
}

pub fn message_template_schedule_build(
    build_id: &str,
    repository_url: &str,
    service_name: &str,
) -> String {
    let current_timestamp = Utc::now().to_rfc3339();

    let message = json!({
        "content": null,
        "embeds": [
          {
            "title": "[1/6] | ⏳ Scheduling new build",
            "description": format!("I'm scheduling a new build for `{}`", build_id),
            "color": 5975807,
            "fields": [
              {
                "name": "Repository URL",
                "value": repository_url,
                "inline": true
              },
              {
                "name": "build_id",
                "value": build_id,
                "inline": true
              },
              {
                "name": "service_name",
                "value": service_name,
                "inline": true
              }
            ],
            "author": {
              "name": "watchdog_rs",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "footer": {
              "text": "Powered by Xylex",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "timestamp": current_timestamp
          }
        ],
        "attachments": []
    });

    message.to_string()
}

pub fn message_template_build_succeeded(
    build_id: &str,
    repository_url: &str,
    service_name: &str,
) -> String {
    let current_timestamp = Utc::now().to_rfc3339();

    let message = json!({
        "content": null,
        "embeds": [
          {
            "title": "[4/6] | ✅ Build succeeded",
            "description": format!("Build `{}` succeeded", build_id),
            "color": 3066993,
            "fields": [
              {
                "name": "Repository URL",
                "value": repository_url,
                "inline": true
              },
              {
                "name": "build_id",
                "value": build_id,
                "inline": true
              },
              {
                "name": "service_name",
                "value": service_name,
                "inline": true
              }
            ],
            "author": {
              "name": "watchdog_rs",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "footer": {
              "text": "Powered by Xylex",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "timestamp": current_timestamp
          }
        ],
        "attachments": []
    });

    message.to_string()
}

pub fn message_template_build_failed(
    build_id: &str,
    repository_url: &str,
    service_name: &str,
) -> String {
    let current_timestamp = Utc::now().to_rfc3339();

    let message = json!({
        "content": null,
        "embeds": [
          {
            "title": "[4/6] | ❌ Build failed",
            "description": format!("Build `{}` failed", build_id),
            "color": 15158332,
            "fields": [
              {
                "name": "Repository URL",
                "value": repository_url,
                "inline": true
              },
              {
                "name": "build_id",
                "value": build_id,
                "inline": true
              },
              {
                "name": "service_name",
                "value": service_name,
                "inline": true
              }
            ],
            "author": {
              "name": "watchdog_rs",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "footer": {
              "text": "Powered by Xylex",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "timestamp": current_timestamp
          }
        ],
        "attachments": []
    });

    message.to_string()
}

pub fn message_template_trying_to_deploy(
    build_id: &str,
    repository_url: &str,
    service_name: &str,
) -> String {
    let current_timestamp = Utc::now().to_rfc3339();

    let message = json!({
        "content": null,
        "embeds": [
          {
            "title": "[5/6] 🚀 Trying to deploy",
            "description": format!("Trying to deploy build `{}`", build_id),
            "color": 15844367,
            "fields": [
              {
                "name": "Repository URL",
                "value": repository_url,
                "inline": true
              },
              {
                "name": "build_id",
                "value": build_id,
                "inline": true
              },
              {
                "name": "service_name",
                "value": service_name,
                "inline": true
              }
            ],
            "author": {
              "name": "watchdog_rs",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "footer": {
              "text": "Powered by Xylex",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "timestamp": current_timestamp
          }
        ],
        "attachments": []
    });

    message.to_string()
}

pub fn message_template_successfully_deployed(
    build_id: &str,
    repository_url: &str,
    service_name: &str,
) -> String {
    let current_timestamp = Utc::now().to_rfc3339();

    let message = json!({
        "content": null,
        "embeds": [
          {
            "title": "[6/6] | ✅ Successfully deployed",
            "description": format!("Successfully deployed build `{}`", build_id),
            "color": 3066993,
            "fields": [
              {
                "name": "Repository URL",
                "value": repository_url,
                "inline": true
              },
              {
                "name": "build_id",
                "value": build_id,
                "inline": true
              },
              {
                "name": "service_name",
                "value": service_name,
                "inline": true
              }
            ],
            "author": {
              "name": "watchdog_rs",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "footer": {
              "text": "Powered by Xylex",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "timestamp": current_timestamp
          }
        ],
        "attachments": []
    });

    message.to_string()
}

pub fn message_template_failed_to_deploy(
    build_id: &str,
    repository_url: &str,
    service_name: &str,
) -> String {
    let current_timestamp = Utc::now().to_rfc3339();

    let message = json!({
        "content": null,
        "embeds": [
          {
            "title": "[5/6] | ❌ Failed to deploy",
            "description": format!("Failed to deploy build `{}`", build_id),
            "color": 15158332,
            "fields": [
              {
                "name": "Repository URL",
                "value": repository_url,
                "inline": true
              },
              {
                "name": "build_id",
                "value": build_id,
                "inline": true
              },
              {
                "name": "service_name",
                "value": service_name,
                "inline": true
              }
            ],
            "author": {
              "name": "watchdog_rs",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "footer": {
              "text": "Powered by Xylex",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "timestamp": current_timestamp
          }
        ],
        "attachments": []
    });

    message.to_string()
}

pub fn message_template_starting_tests(
    build_id: &str,
    repository_url: &str,
    service_name: &str,
) -> String {
    let current_timestamp = Utc::now().to_rfc3339();

    let message = json!({
        "content": null,
        "embeds": [
          {
            "title": "[2/6] | 🧪 Starting tests",
            "description": format!("Starting tests for build `{}`", build_id),
            "color": 3447003,
            "fields": [
              {
                "name": "Repository URL",
                "value": repository_url,
                "inline": true
              },
              {
                "name": "build_id",
                "value": build_id,
                "inline": true
              },
              {
                "name": "service_name",
                "value": service_name,
                "inline": true
              }
            ],
            "author": {
              "name": "watchdog_rs",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "footer": {
              "text": "Powered by Xylex",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "timestamp": current_timestamp
          }
        ],
        "attachments": []
    });

    message.to_string()
}

pub fn message_template_tests_passed(
    build_id: &str,
    repository_url: &str,
    service_name: &str,
) -> String {
    let current_timestamp = Utc::now().to_rfc3339();

    let message = json!({
        "content": null,
        "embeds": [
          {
            "title": "[3/6] | ✅ Tests passed",
            "description": format!("Tests passed for build `{}`", build_id),
            "color": 3066993,
            "fields": [
              {
                "name": "Repository URL",
                "value": repository_url,
                "inline": true
              },
              {
                "name": "build_id",
                "value": build_id,
                "inline": true
              },
              {
                "name": "service_name",
                "value": service_name,
                "inline": true
              }
            ],
            "author": {
              "name": "watchdog_rs",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "footer": {
              "text": "Powered by Xylex",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "timestamp": current_timestamp
          }
        ],
        "attachments": []
    });

    message.to_string()
}

pub fn message_template_tests_failed(
    build_id: &str,
    repository_url: &str,
    service_name: &str,
) -> String {
    let current_timestamp = Utc::now().to_rfc3339();

    let message = json!({
        "content": null,
        "embeds": [
          {
            "title": "[3/6] | ❌ Tests failed",
            "description": format!("Tests failed for build `{}`", build_id),
            "color": 15158332,
            "fields": [
              {
                "name": "Repository URL",
                "value": repository_url,
                "inline": true
              },
              {
                "name": "build_id",
                "value": build_id,
                "inline": true
              },
              {
                "name": "service_name",
                "value": service_name,
                "inline": true
              }
            ],
            "author": {
              "name": "watchdog_rs",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "footer": {
              "text": "Powered by Xylex",
              "icon_url": "https://xylex.ams3.cdn.digitaloceanspaces.com/profilePics/watchdog_rs.png"
            },
            "timestamp": current_timestamp
          }
        ],
        "attachments": []
    });

    message.to_string()
}
