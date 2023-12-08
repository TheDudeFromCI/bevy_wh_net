use serde::{Deserialize, Serialize};

use crate::impl_packet;

#[derive(Debug, Serialize, Deserialize)]
pub struct KickPacket {
    pub reason: String,
}
impl_packet!(to_client KickPacket);
