mod loader;

#[actix_web::main]
async fn main() {
    println!("Hello, world!");
    loader::load_and_start().await;
}