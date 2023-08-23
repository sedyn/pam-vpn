use std::{process::Command, time::Duration};

use rexpect::session::spawn_command;

use crate::watcher::{wait_until_file_created, wait_until_string_found};

use super::{credential::UserPasswordCredential, VPNClient};

pub struct OpenVPNClient<'a> {
    pub config_path: Option<&'a str>,
}

const LOG_FILE_PATH: &str = "/var/log/openvpn.log";
const DEFAULT_TIMEOUT_SECS: u64 = 5;
const DEFAULT_SLEEP_MILLIS: u64 = 100;

impl VPNClient<UserPasswordCredential<'_>> for OpenVPNClient<'_> {
    fn connect(&self, credential: UserPasswordCredential) -> anyhow::Result<()> {
        let mut command = Command::new("openvpn");

        command
            .arg("--daemon")
            .arg("--config")
            .arg(self.config_path.unwrap_or("/etc/pam.d/client.conf"))
            .arg("--log")
            .arg(LOG_FILE_PATH);

        let mut session = spawn_command(command, None)?;

        session.exp_string("Enter Auth Username")?;
        session.send_line(credential.username)?;
        session.exp_string("Enter Auth Password")?;
        session.send_line(credential.password)?;

        let file = wait_until_file_created(
            LOG_FILE_PATH,
            Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            Duration::from_millis(DEFAULT_SLEEP_MILLIS),
        )?;

        let found = wait_until_string_found(
            file,
            "Initialization Sequence Completed",
            Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            Duration::from_millis(DEFAULT_SLEEP_MILLIS),
        )?;

        match found {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("failed to connect openVPN")),
        }
    }
}

impl<'a> OpenVPNClient<'a> {}
