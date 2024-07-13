use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde_json::json;
use std::collections::HashMap;
use std::env::var;
use std::io::Result;

#[get("/ping")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Hello world!"}))
}

#[post("/build")]
async fn build(query: web::Query<HashMap<String, String>>) -> impl Responder {
    dotenv().ok();
    let required_build_key: String = var("BUILD_KEY").expect("BUILD_KEY must be set");

    let default_build_id: String = "0".to_string();
    let build_id: &String = query.get("build_id").unwrap_or(&default_build_id);

    if let Some(build_key) = query.get("build_key") {
        if build_key == &required_build_key {
            return HttpResponse::Ok().json(json!(
                {"message": "buidl!", "build_id": build_id}
            ));
        }
    }
    HttpResponse::Unauthorized().json(json!(
        {"error": "Unauthorized"}
    ))
}

#[actix_web::main]
async fn main() -> Result<()> {
    println!("\x1b[1;32mServer running\x1b[0m");

    HttpServer::new(|| App::new().service(index).service(build))
        .bind(("127.0.0.1", 4035))?
        .run()
        .await
}
