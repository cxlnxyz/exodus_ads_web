use actix_web::{HttpResponse, Responder};
use actix_session::Session;
use crate::loader::server::ldap::get_ad_users;
use std::fs::read_to_string;

pub async fn dashboard(session: Session) -> impl Responder {
    if let Some(user) = session.get::<String>("user").unwrap() {
        let content = include_str!("../../sites/dashboard.html").replace("{{username}}", &user);
        HttpResponse::Ok().content_type("text/html").body(content)
    } else {
        let content = read_to_string("src/sites/unauthorized.html").unwrap();
        HttpResponse::Unauthorized().content_type("text/html").body(content)
    }
}

pub async fn get_users() -> impl Responder {
    match get_ad_users() {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout).to_string();
            HttpResponse::Ok().content_type("application/json").body(output_str)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error fetching users: {}", e))
        }
    }
}