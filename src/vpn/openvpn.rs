use std::process::Command;

use super::{credential::UserPasswordCredential, VPNClient};

pub struct OpenVPNClient;

impl VPNClient<UserPasswordCredential<'_>> for OpenVPNClient {
    fn connect(&self, credential: UserPasswordCredential) -> anyhow::Result<()> {
        let output = Command::new("openvpn")
            .arg("--username ")
            .arg(credential.user)
            .arg("--password")
            .arg(credential.password)
            .output()?;

        
        
        Ok(())
    }
}
