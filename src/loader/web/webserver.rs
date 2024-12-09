use crate::loader::web::dashboard::dashboard;
use crate::loader::web::login::login;
use actix_session::{CookieSession, Session};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs::read_to_string;

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

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .route("/", web::get().to(index))
            .route("/login", web::get().to(login_page))
            .route("/login", web::post().to(login))
            .route("/dashboard", web::get().to(dashboard))
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