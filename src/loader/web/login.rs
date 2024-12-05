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
    match authenticate(&form.username, &form.password) {
        Ok(true) => {
            session.insert("user", &form.username).unwrap();
            let response = LoginResponse {
                success: true,
                message: None,
            };
            HttpResponse::Ok().json(response)
        }
        Ok(false) => {
            let response = LoginResponse {
                success: false,
                message: Some("Invalid credentials".to_string()),
            };
            HttpResponse::Ok().json(response)
        }
        Err(_) => {
            let response = LoginResponse {
                success: false,
                message: Some("Error connecting to AD".to_string()),
            };
            HttpResponse::Ok().json(response)
        }
    }
}

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}