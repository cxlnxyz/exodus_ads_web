use crate::loader::web::dashboard::{dashboard, get_users};
use crate::loader::web::login::login;
use actix_session::{CookieSession, Session};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs::read_to_string;
use crate::loader::server::ldap::get_ad_users;
use crate::loader::server::sync::{run_syncall, get_recent_syncs};

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

async fn get_user_count() -> impl Responder {
    match get_ad_users() {
        Ok(output) => {
            let users: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout).unwrap();
            HttpResponse::Ok().json(users.len())
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching user count: {}", e)),
    }
}

async fn get_system_count() -> impl Responder {
    // Replace with actual logic to count systems
    let system_count = 1;
    HttpResponse::Ok().json(system_count)
}

async fn get_recent_syncs_handler() -> impl Responder {
    match web::block(|| get_recent_syncs()).await {
        Ok(syncs) => match syncs {
            Ok(syncs) => HttpResponse::Ok().json(syncs),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching recent syncs: {}", e)),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error blocking thread: {}", e)),
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
            .route("/dashboard/sync", web::get().to(sync_ad))
            .route("/dashboard/user_count", web::get().to(get_user_count))
            .route("/dashboard/system_count", web::get().to(get_system_count))
            .route("/dashboard/recent_syncs", web::get().to(get_recent_syncs_handler))
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