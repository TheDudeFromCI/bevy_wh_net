use bevy_wh_net_derive::{packet_to_client, PacketCore};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PacketCore)]
#[packet_to_client]
pub struct HandshakePacket;
