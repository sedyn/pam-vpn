use std::{process::{Command, Stdio}, io::Read};

use anyhow::Context;

use super::{credential::UserPasswordCredential, VPNClient};

pub struct OpenVPNClient<'a> {
    pub config_path: Option<&'a str>,
}

impl VPNClient<UserPasswordCredential<'_>> for OpenVPNClient<'_> {
    fn connect(&self, credential: UserPasswordCredential) -> anyhow::Result<()> {
        let mut child = Command::new("openvpn")
            //.arg("--daemon")
            .arg("--config")
            .arg(self.config_path.unwrap_or("/etc/pam.d/client.conf"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        // let stdin = child
        //     .stdin
        //     .as_mut()
        //     .context("unable to get stdin of child")?;

        // credential.write(stdin)?;
        // drop(stdin);

        let stdout = child
            .stdout
            .as_mut()
            .context("unable to get stdout of child")?;

        let mut buf= String::new();
        while let Ok(size) = stdout.read_to_string(&mut buf) {
            if size > 0 {
                println!("{}", &buf);
            }
        }

        Ok(())
    }
}

impl<'a> OpenVPNClient<'a> {

}
