use actix_web::{HttpResponse, Responder};
use actix_session::Session;
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