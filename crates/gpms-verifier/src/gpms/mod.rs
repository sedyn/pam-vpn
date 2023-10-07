use serde::Serialize;

struct GPMSClient {
    host: String,
    client: reqwest::blocking::Client
}

impl GPMSClient {
    fn user_authentication(&self, username: &'_ str, password: &'_ str, machine_id: &'_ str) {
        let request = UserAuthenticationRequest {
            auth_type: "IDPW",
            username,
            password,
            machine_id,
        };

        let response = self.client
            .post(format!("{}/glm/v1/api/outgw/auth", &self.host))
            .json(&request)
            .send();
    }
}

#[derive(Serialize)]
struct UserAuthenticationRequest<'a> {
    #[serde(rename = "authType")]
    auth_type: &'a str,
    #[serde(rename = "UserID")]
    username: &'a str,
    #[serde(rename = "UserPWD")]
    password: &'a str,
    #[serde(rename = "MachineID")]
    machine_id: &'a str,
}
