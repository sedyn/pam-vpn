use pam::constants::{PamResultCode, PAM_PROMPT_ECHO_OFF};
use pam::conv::Conv;
use pam::module::PamHooks;
use pam::pam_try;
use std::ffi::CStr;

use crate::vpn::credential::UserPasswordCredential;
use crate::vpn::openvpn::OpenVPNClient;
use crate::vpn::VPNClient;

mod vpn;

struct PamVPN;
pam::pam_hooks!(PamVPN);

impl PamHooks for PamVPN {
    fn sm_authenticate(
        pamh: &mut pam::module::PamHandle,
        args: Vec<&std::ffi::CStr>,
        flags: pam::constants::PamFlag,
    ) -> pam::constants::PamResultCode {
        dbg!(args);

        let user = pam_try!(pamh.get_user(None));

        let conv = match pamh.get_item::<Conv>() {
            Ok(Some(conv)) => conv,
            Ok(None) => unreachable!("No conv available"),
            Err(err) => {
                return err;
            }
        };

        let password = pam_try!(conv.send(PAM_PROMPT_ECHO_OFF, "password: "));
        let password = match password {
            Some(password) => pam_try!(password.to_str(), PamResultCode::PAM_AUTH_ERR),
            None => {
                return PamResultCode::PAM_AUTH_ERR;
            }
        };

        let credential = UserPasswordCredential {
            user: &user,
            password,
        };

        dbg!(&credential);

        let client = OpenVPNClient {
            config_path: None
        };

        match client.connect(credential) {
            Ok(_) => PamResultCode::PAM_SUCCESS,
            Err(e) => {
                dbg!(e);
                PamResultCode::PAM_AUTH_ERR
            }
        }
    }
}
