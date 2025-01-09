use crate::loader::web::dashboard::{dashboard, get_users};
use crate::loader::web::login::login;
use actix_session::{CookieSession, Session};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs::read_to_string;
use crate::loader::server::sync::run_syncall;

async fn index() -> impl Responder {
    let content = read_to_string("src/sites/index.html").unwrap();
    HttpResponse::Ok().content_type("text/html").body(content)
}

async fn login_page() -> impl Responder {
    let content = read_to_string("src/sites/login.html").unwrap();
    HttpResponse::Ok().content_type("text/html").body(content)
}

async fn logout(session: Session) -> impl Responder {
    session.clear();
    HttpResponse::Found().header("Location", "/login").finish()
}

async fn sync_ad() -> impl Responder {
    let dc1_ip = "CN-DC1.COLIN.HOME"; // Replace with your DC1 FQDN
    let dc2_ip = "CN-DC2.COLIN.HOME"; // Replace with your DC2 FQDN

    match run_syncall(dc1_ip, dc2_ip) {
        Ok(_) => HttpResponse::Ok().body("Synchronization successful"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error during synchronization: {}", e)),
    }
}

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .route("/", web::get().to(index))
            .route("/login", web::get().to(login_page))
            .route("/login", web::post().to(login))
            .route("/dashboard", web::get().to(dashboard))
            .route("/dashboard/users", web::get().to(get_users))
            .route("/dashboard/sync", web::get().to(sync_ad)) // Add this line
            .route("/logout", web::get().to(logout))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

pub async fn load() {
    println!("Webserver loaded!");
    start_server().await.unwrap();
}