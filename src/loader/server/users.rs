use ldap3::{LdapConnAsync, Scope, SearchEntry};
use std::error::Error;

pub async fn authenticate(username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
    let ldap_url = "ldap://colin-CN-DC2-CA.home";
    let bind_dn = format!("CN={},DC=colin,DC=home", username);
    let search_base = "DC=colin,DC=home";
    let search_filter = format!("(sAMAccountName={})", username);

    let (conn, mut ldap) = LdapConnAsync::new(ldap_url).await?;
    ldap3::drive!(conn);

    let bind_result = ldap.simple_bind(&bind_dn, password).await?.success();
    if bind_result.is_err() {
        return Ok(false);
    }

    let search_result = ldap
        .search(
            search_base,
            Scope::Subtree,
            &search_filter,
            vec!["dn"],
        )
        .await?
        .success()?;

    let entries: Vec<SearchEntry> = search_result
        .into_iter()
        .map(SearchEntry::construct)
        .collect();

    Ok(!entries.is_empty())
}