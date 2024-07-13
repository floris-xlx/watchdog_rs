use actix_web::{App, HttpServer};
use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use std::env::var;
use std::io::Result;
use watchdog_rs::api::client::{build, index};

fn set_progress_message(progress: &ProgressBar, message: &str) {
    progress.set_message(message.to_string());
    progress.inc(1);  // Ensure the progress bar updates immediately with the new message
}

#[actix_web::main]
async fn main() -> Result<()> {
    let progress: ProgressBar = ProgressBar::new(100);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{bar:40.cyan/blue} {msg}")
            .expect("Failed to set progress bar template"),
    );
    set_progress_message(&progress, "Initializing server");

    let default_port: u16 = 4035;
    dotenv().ok();

    let port: u16 = var("WATCHDOG_RS_PORT")
        .unwrap_or(default_port.to_string())
        .parse()
        .expect("WATCHDOG_RS_PORT must be a valid u16");

    if var("WATCHDOG_RS_PORT").is_err() {
        let message = format!("WATCHDOG_RS_PORT ENV VAR wasn't set, defaulting to {}", default_port);
        set_progress_message(&progress, &message);
    } else {
        let message = format!("Server attempting to bind to port {}", port);
        set_progress_message(&progress, &message);
    }

    let server = HttpServer::new(|| App::new().service(index).service(build))
        .bind(("127.0.0.1", port))?
        .run();

    let message = format!("✔️   Server successfully started on port {}", port);
    progress.finish_with_message(format!("{}", message));
    println!("watchdog_rs: Listening for requests...");

    server.await
}
