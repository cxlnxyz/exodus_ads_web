use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: Option<String>,
}

pub async fn login(session: Session, form: web::Form<LoginForm>) -> impl Responder {
    if form.username == "admin" && form.password == "admin" {
        session.insert("user", "admin").unwrap();
        let response = LoginResponse {
            success: true,
            message: None,
        };
        HttpResponse::Ok().json(response)
    } else {
        let response = LoginResponse {
            success: false,
            message: Some("Invalid credentials".to_string()),
        };
        HttpResponse::Ok().json(response)
    }
}

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}