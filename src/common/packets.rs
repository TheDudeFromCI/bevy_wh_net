use std::any::Any;

#[typetag::serde]
pub trait Packet: Any + Send + Sync + std::fmt::Debug {
    fn can_send_to_client(&self) -> bool;
    fn can_send_to_server(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl<P: Packet> From<P> for PacketContainer {
    fn from(packet: P) -> Self {
        Self::new(packet)
    }
}

#[derive(Debug)]
pub struct PacketContainer {
    packet: Box<dyn Packet>,
}

impl PacketContainer {
    pub fn new(packet: impl Packet) -> Self {
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
        T: Packet,
    {
        self.packet.as_any().downcast_ref::<T>()
    }
}

#[macro_export]
macro_rules! impl_packet {
    (to_client $t:ty) => {
        #[typetag::serde]
        impl $crate::common::networking::Packet for $t {
            fn can_send_to_client(&self) -> bool {
                true
            }

            fn can_send_to_server(&self) -> bool {
                false
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    (to_server $t:ty) => {
        #[typetag::serde]
        impl $crate::common::networking::Packet for $t {
            fn can_send_to_client(&self) -> bool {
                false
            }

            fn can_send_to_server(&self) -> bool {
                true
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    (to_both $t:ty) => {
        #[typetag::serde]
        impl $crate::common::networking::Packet for $t {
            fn can_send_to_client(&self) -> bool {
                true
            }

            fn can_send_to_server(&self) -> bool {
                true
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}
