use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};

pub async fn login(session: Session, form: web::Form<LoginForm>) -> impl Responder {
    if form.username == "admin" && form.password == "admin" {
        session.insert("user", "admin").unwrap();
        HttpResponse::Found().header("Location", "/dashboard").finish()
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}