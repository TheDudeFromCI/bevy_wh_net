use std::env;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref PROTOCOL_ID: u64 = get_protocol_id();
}

fn get_protocol_id() -> u64 {
    let major = env!("CARGO_PKG_VERSION_MAJOR");
    let minor = env!("CARGO_PKG_VERSION_MINOR");
    let patch = env!("CARGO_PKG_VERSION_PATCH");
    format_protocol_id(major, minor, patch)
}

fn format_protocol_id(major: &str, minor: &str, patch: &str) -> u64 {
    let major = major.parse::<u64>().unwrap();
    let minor = minor.parse::<u64>().unwrap();
    let patch = patch.parse::<u64>().unwrap();

    (major << 32) | (minor << 16) | patch
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_from_version() {
        let major = "1";
        let minor = "27";
        let patch = "3";

        let id = format_protocol_id(major, minor, patch);

        assert_eq!(id, 0x0000_0001_001b_0003u64);
    }
}
