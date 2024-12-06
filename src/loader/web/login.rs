use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use log::error;
use crate::loader::server::users::authenticate;

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: Option<String>,
}

pub async fn login(session: Session, form: web::Form<LoginForm>) -> impl Responder {
    match authenticate(&form.username, &form.password).await {
        Ok(true) => {
            if let Err(e) = session.insert("user", &form.username) {
                error!("Failed to insert session: {}", e);
                let response = LoginResponse {
                    success: false,
                    message: Some("Session error".to_string()),
                };
                return HttpResponse::InternalServerError().json(response);
            }
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
        Err(e) => {
            error!("Error connecting to AD: {}", e);
            let response = LoginResponse {
                success: false,
                message: Some("Error connecting to AD".to_string()),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}