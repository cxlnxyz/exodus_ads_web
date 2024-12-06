use ldap3::{LdapConn, Scope, SearchEntry};
use std::error::Error;

pub fn authenticate(username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
    let mut ldap = LdapConn::new("ldap://colin.home:389")?;

    // Try different formats for the username
    let bind_formats = vec![
        format!("CN={},CN=Users,DC=colin,DC=home", username),
        format!("{}@colin.home", username),
        username.to_string(),
    ];

    for bind_dn in bind_formats {
        println!("Trying to bind with: {}", bind_dn);
        match ldap.simple_bind(&bind_dn, password) {
            Ok(_) => {
                println!("Successfully bound with: {}", bind_dn);
                // If the bind was successful, perform a search
                let result = ldap.search(
                    "DC=colin,DC=home",
                    Scope::Subtree,
                    &format!("(sAMAccountName={})", username),
                    vec!["cn"]
                )?;

                return Ok(!result.0.is_empty());
            },
            Err(e) => println!("Failed to bind with {}: {:?}", bind_dn, e),
        }
    }

    Ok(false)
}