use std::process::Command;

use anyhow::Context;

use super::{credential::UserPasswordCredential, VPNClient};

pub struct OpenVPNClient<'a> {
    pub config_path: Option<&'a str>,
}

impl VPNClient<UserPasswordCredential<'_>> for OpenVPNClient<'_> {
    fn connect(&self, credential: UserPasswordCredential) -> anyhow::Result<()> {
        let mut child = Command::new("openvpn")
            .arg("--config")
            .arg(self.config_path.unwrap_or("/etc/pam.d/client.conf"))
            .arg("--auth-user-pass")
            .spawn()?;

        let stdin = child
            .stdin
            .as_mut()
            .context("unable to acquire stdin of child")?;

        credential.write(stdin)?;

        let output = child.wait_with_output()?;

        println!("{:?}", output.status);

        Ok(())
    }
}
