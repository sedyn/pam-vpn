use std::process::Command;

use rexpect::session::spawn_command;

use super::{credential::UserPasswordCredential, VPNClient};

pub struct OpenVPNClient<'a> {
    pub config_path: Option<&'a str>,
}

impl VPNClient<UserPasswordCredential<'_>> for OpenVPNClient<'_> {
    fn connect(&self, credential: UserPasswordCredential) -> anyhow::Result<()> {
        let mut command = Command::new("openvpn");

        command
            .arg("--daemon")
            .arg("--config")
            .arg(self.config_path.unwrap_or("/etc/pam.d/client.conf"));

        let mut session = spawn_command(command, None)?;

        session.exp_string("Enter Auth Username")?;
        session.send_line(credential.username)?;
        session.exp_string("Enter Auth Password")?;
        session.send_line(credential.password)?;

        Ok(())
    }
}

impl<'a> OpenVPNClient<'a> {}
