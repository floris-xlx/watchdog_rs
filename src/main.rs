use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env::var;
use std::io::Result;

use watchdog_rs::api::client::{build, index};

#[actix_web::main]
async fn main() -> Result<()> {
    println!("\x1b[1;32mwatchdog_rs: Initializing server\x1b[0m");
    let default_port: u16 = 4035;

    dotenv().ok();

    let port: u16 = var("PORT")
        .unwrap_or(default_port.to_string())
        .parse()
        .expect("PORT must be a valid u16");

    let port_message: String = format!(
        "\x1b[1;33mPORT ENV VAR wasn't set, defaulting to \x1b[1;33m{}\x1b[0m",
        default_port
    );

    if var("PORT").is_err() {
        println!("watchdog_rs: {}", port_message);
    } else {
        println!(
            "\x1b[1;34mwatchdog_rs: Server attempting to bind to port {}\x1b[0m",
            port
        );
    }

    let server = HttpServer::new(|| App::new().service(index).service(build))
        .bind(("127.0.0.1", port))?
        .run();

    println!(
        "\x1b[1;32mwatchdog_rs: Server successfully started on port {}\x1b[0m",
        port
    );

    server.await
}
