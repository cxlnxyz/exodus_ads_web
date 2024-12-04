mod server {
    pub mod ldap;
    pub mod users;
}

mod web {
    pub mod webserver;
    pub mod login;
    pub mod dashboard;
    pub mod middleware;
}

pub fn load_and_start() {
    println!("Loading LDAP module...");
    server::ldap::some_function();

    println!("Loading Users module...");
    server::users::some_function();

    println!("Loading Webserver module...");
    web::webserver::load();
}