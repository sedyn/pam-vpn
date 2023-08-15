pub mod credential;
pub mod openvpn;

pub trait VPNClient<T> {
    fn connect(&self, credential: T) -> anyhow::Result<()>;
}
