use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use crate::loader::server::users::LoginRequest;
use crate::loader::server::users::ldap_login;

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: Option<String>,
}

pub async fn login(session: Session, form: web::Form<LoginForm>) -> impl Responder {
    println!("Received login request for username: {}", form.username);
    match ldap_login(web::Json(LoginRequest {
        username: form.username.clone(),
        password: form.password.clone(),
    })).await {
        Ok(response) => {
            println!("Login response: {:?}", response);
            if response.status().is_success() {
                session.insert("user", &form.username).unwrap();
                HttpResponse::Found().header("Location", "/dashboard").finish()
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        },
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