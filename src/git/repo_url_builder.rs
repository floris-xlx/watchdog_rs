use dotenv::dotenv;
use regex::Regex;
use std::env::var;
use std::error::Error;

// crate imports
use crate::utils::print;
use crate::config::{get_webhook_url_by_repository, parse_watchdog_rs_yml};
use crate::log::discord_log_webhook;

/// Builds a repository URL with an API key for private repositories
pub async fn repository_url_builder(repository_url: &str, private: bool) -> String {
    let mut url: String = repository_url.to_string();

    // get the webhook URL from the repository URL
    let webhook_url: String = match get_webhook_url_by_repository(
        parse_watchdog_rs_yml("watchdog_rs.yml").unwrap(),
        &url,
    ) {
        Some(url) => url,
        None => {
            print::print_red("Failed to retrieve webhook URL from repository URL");
            return url.to_string();
        },
    };

    dotenv().ok();

    let api_key: String = match var("WATCHDOG_RS_GITHUB_KEY") {
        Ok(key) => key,
        Err(_) => {
            print::print_red("Failed to retrieve WATCHDOG_RS_GITHUB_KEY from environment variables");
            return url;
        },
    };

    if private {
        let re: Regex = match Regex::new(r"https://github.com/(.*)") {
            Ok(regex) => regex,
            Err(_) => {
                print::print_red("Failed to compile regex for repository URL");

                let log_result: Result<(), Box<dyn Error>> = discord_log_webhook("Failed to compile regex for repository URL", &webhook_url).await;

                if log_result.is_err() {
                    print::print_red("Failed to send log message to Discord");
                }

                return url;
            },
        };
        if let Some(captures) = re.captures(&url) {
            let repo_path = &captures[1];
            url = format!("https://{}@github.com/{}", api_key, repo_path);
        }
    }

    url
}
