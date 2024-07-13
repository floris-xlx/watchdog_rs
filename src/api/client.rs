use actix_web::{get, post, web, HttpResponse, Responder};
use dotenv::dotenv;
use serde_json::json;
use std::collections::HashMap;
use std::env::var;


#[get("/ping")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Hello world!"}))
}

#[post("/build")]
pub async fn build(query: web::Query<HashMap<String, String>>) -> impl Responder {
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
