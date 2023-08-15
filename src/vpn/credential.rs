use std::io::Write;

#[derive(Debug)]
pub struct UserPasswordCredential<'a> {
    pub user: &'a str,
    pub password: &'a str,
}

impl<'a> UserPasswordCredential<'a> {
    pub fn write<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        writer.write(self.user.as_bytes())?;
        writer.write(b"\n")?;
        writer.write(self.password.as_bytes())?;
        writer.write(b"\n")?;
        writer.flush()?;
        Ok(())
    }
}
