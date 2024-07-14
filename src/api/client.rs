use actix_web::{get, post, web, HttpResponse, Responder};
use dotenv::dotenv;
use serde_json::{json, Value};

use crate::config::parse_watchdog_rs_yml;
use crate::git::repo_url_builder::repository_url_builder;
use crate::utils::print;
use crate::api::parsing::{
    extract_first_event, extract_nested_param, extract_param, is_authorized,
};

#[get("/ping")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Hello world!"}))
}

#[post("/build")]
pub async fn build(
    params: web::Query<serde_json::Value>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    dotenv().ok();

    println!(
        "\x1b[34mwatchdog_rs | Incoming git request: \n{:#?}\x1b[0m",
        params
    );

    let build_key: Option<&str> = extract_param(&params, &body, "build_key");
    let build_id: &str = extract_param(&params, &body, "build_id").unwrap_or("0");
    let repository_url: Option<&str> =
        extract_nested_param(&params, &body, "repository", "html_url");
    let first_event: Option<&serde_json::Value> =
        extract_first_event(&params, &body, "hook", "events", "push");

    let private: bool = match body.get("repository").and_then(|repo| repo.get("private")) {
        Some(Value::Bool(value)) => *value,
        Some(Value::String(value)) => match value.parse::<bool>() {
            Ok(parsed_value) => parsed_value,
            Err(_) => {
                print::print_red("🚨 Failed to parse repository privacy status from request");
                return HttpResponse::BadRequest()
                    .json(json!({"error": "Bad Request"}));
            }
        },
        _ => {
            print::print_red("🚨 Failed to retrieve repository privacy status from request");
            return HttpResponse::BadRequest()
                .json(json!({"error": "Bad Request"}));
        }
    };

    println!(
        "\x1b[34mwatchdog_rs | Parsed git request: \nbuild_key: {:?}\nbuild_id: {:?}\nrepository_url: {:?}\nfirst_event: {:?}\x1b[0m",
        build_key, build_id, repository_url, first_event);

    match parse_watchdog_rs_yml("watchdog_rs.yml") {
        Ok(services) => {
            for (service_name, service_config) in services {
                if is_authorized(&service_config, build_key, repository_url, first_event) {
                    if build_id == service_config.WATCHDOG_RS_BUILD_ID {

                        let repository_url: String = match repository_url {
                            Some(url) => url.to_string(),
                            None => {
                                print::print_red("🚨 Failed to retrieve repository URL from request");
                                return HttpResponse::BadRequest()
                                    .json(json!({"error": "Bad Request"}));
                            }
                        };


                        let url_with_key: String = repository_url_builder(&repository_url, private).await;

                        return HttpResponse::Ok().json(json!(
                            {
                                "message": "build!",
                                "build_id": build_id,
                                "repository_url": repository_url,
                                "service_name": service_name,
                                "repository_url_with_key": url_with_key,
                                // "is_private": is_private,
                                "private":  private
                            }
                        ));
                    } else {
                        return HttpResponse::Unauthorized().json(json!(
                            {"error": "Unauthorized", "details": "Invalid build_id"}
                        ));
                    }
                }
            }
        }
        Err(e) => eprintln!("Error parsing YAML file: {}", e),
    }

    println!("\x1b[31mwatchdog_rs: 🔒 Unauthorized build_key tried requesting\x1b[0m");

    HttpResponse::Unauthorized().json(json!(
        {"error": "Unauthorized"}
    ))
}
