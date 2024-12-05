use ldap3::{LdapConn, LdapError, Scope};

pub fn load() {
    println!("users function called");
}

pub fn authenticate(username: &str, password: &str) -> Result<bool, LdapError> {
    let ldap_url = "ldap://your-ad-server";
    let bind_dn = format!("{}@your-domain", username);
    let search_base = "dc=your-domain,dc=com";
    let search_filter = format!("(sAMAccountName={})", username);

    let mut ldap = LdapConn::new(ldap_url)?;
    ldap.simple_bind(&bind_dn, password)?.success()?;

    let (rs, _res) = ldap.search(
        search_base,
        Scope::Subtree,
        &search_filter,
        vec!["dn"],
    )?.success()?;

    if rs.len() == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}