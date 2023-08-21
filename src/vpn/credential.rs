#[derive(Debug)]
pub struct UserPasswordCredential<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
