use pam::module::PamHooks;
use pam::constants::PamResultCode;

mod vpn;

struct PamVPN;
pam::pam_hooks!(PamVPN);

impl PamHooks for PamVPN {

    fn sm_authenticate(pamh: &mut pam::module::PamHandle, args: Vec<&std::ffi::CStr>, flags: pam::constants::PamFlag) -> pam::constants::PamResultCode {
        dbg!(args);
        println!("test");
        PamResultCode::PAM_SUCCESS
    }

}
