use std::any::Any;

#[typetag::serde(tag = "type", content = "value")]
pub trait PacketImpl: Any + Send + Sync + std::fmt::Debug {
    fn can_send_to_client(&self) -> bool;
    fn can_send_to_server(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl<P: PacketImpl> From<P> for PacketContainer {
    fn from(packet: P) -> Self {
        Self::new(packet)
    }
}

#[derive(Debug)]
pub struct PacketContainer {
    packet: Box<dyn PacketImpl>,
}

impl PacketContainer {
    pub fn new(packet: impl PacketImpl) -> Self {
        Self {
            packet: Box::new(packet),
        }
    }

    pub fn serialize(&self) -> Option<Vec<u8>> {
        bincode::serialize(&self.packet).ok()
    }

    pub fn deserialize(buffer: &[u8]) -> Option<Self> {
        bincode::deserialize(buffer)
            .ok()
            .map(|packet| Self { packet })
    }

    pub fn as_packet<T>(&self) -> Option<&T>
    where
        T: PacketImpl,
    {
        self.packet.as_any().downcast_ref::<T>()
    }
}
