mod server {
    pub mod ldap;
    pub mod users;
    pub mod sync;
}

mod web {
    pub mod webserver;
    pub mod login;
    pub mod dashboard;
    pub mod middleware;
}

pub fn load_and_start() {
    println!("Loading Webserver module...");
    web::webserver::load();
}