use std::{
    env::temp_dir,
    fs::{remove_file, File},
    io::Write,
    process::Command,
};

use anyhow::Context;

use super::{credential::UserPasswordCredential, VPNClient};

pub struct OpenVPNClient;

impl VPNClient<UserPasswordCredential<'_>> for OpenVPNClient {
    fn connect(&self, credential: UserPasswordCredential) -> anyhow::Result<()> {
        let mut child = Command::new("openvpn")
            .arg("--config")
            .arg("client.conf")
            .arg("--auth-user-pass")
            .spawn()?;

        let stdin = child
            .stdin
            .as_mut()
            .context("unable to acquire stdin of child")?;

        credential.write(stdin)?;

        Ok(())
    }
}
