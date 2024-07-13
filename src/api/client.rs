use actix_web::{get, post, web, HttpResponse, Responder};
use dotenv::dotenv;
use serde_json::json;
use std::env::var;

use crate::config::ServiceConfig;
use crate::config::parse_watchdog_rs_yml;


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

    println!("\x1b[34mwatchdog_rs | Incoming git request: \n{:#?}\x1b[0m", params);

    let build_key: Option<&str> = params.get("build_key")
        .or_else(|| body.get("build_key"))
        .and_then(|v| v.as_str());
    let build_id: &str = params.get("build_id")
        .or_else(|| body.get("build_id"))
        .and_then(|v| v.as_str())
        .unwrap_or("0");

    let events = params.get("hook")
        .or_else(|| body.get("hook"))
        .and_then(|hook| hook.get("events"))
        .and_then(|events| events.as_array());
    let first_event = events.and_then(|events| events.iter().find(|&event| event.as_str() == Some("push")));

    let repository_url = params.get("repository")
        .or_else(|| body.get("repository"))
        .and_then(|repository| repository.get("html_url"))
        .and_then(|repository_url| repository_url.as_str());

    match parse_watchdog_rs_yml("watchdog_rs.yml") {
        Ok(services) => {
            for (service_name, service_config) in services {
                if build_key == Some(service_config.WATCHDOG_RS_BUILD_KEY.as_str()) 
                    && repository_url == Some(service_config.WATCHDOG_RS_REPOSITORY_URL.as_str()) 
                    && first_event.is_some() {
                    if build_id == service_config.WATCHDOG_RS_BUILD_ID {
                        return HttpResponse::Ok().json(json!(
                            {"message": "buidl!", "build_id": build_id}
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
