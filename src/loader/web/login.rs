use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use crate::loader::server::users::authenticate;

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: Option<String>,
}

pub async fn login(session: Session, form: web::Form<LoginForm>) -> impl Responder {
    println!("Received login request for username: {}", form.username);
    match authenticate(&form.username, &form.password).await {
        Ok(true) => {
            println!("Login successful for user: {}", form.username);
            HttpResponse::Ok().body("Login erfolgreich")
        }
        Ok(false) => {
            println!("Login failed for user: {}", form.username);
            HttpResponse::Ok().body("Login fehlgeschlagen")
        }
        Err(e) => {
            println!("Authentication error: {}", e);
            HttpResponse::InternalServerError().body("Authentication error")
        }
    }
}

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}