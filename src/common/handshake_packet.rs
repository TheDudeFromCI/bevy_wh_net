use serde::{Deserialize, Serialize};

use crate::impl_packet;

#[derive(Debug, Serialize, Deserialize)]
pub struct HandshakePacket;
impl_packet!(to_client HandshakePacket);
