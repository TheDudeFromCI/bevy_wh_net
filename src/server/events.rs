use bevy::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Event, Clone)]
pub struct ClientConnectedEvent {
    pub client_id: u64,
}

#[derive(Debug, Event, Clone)]
pub struct ClientDisconnectedEvent {
    pub client_id: u64,
}

#[derive(Debug, Event, Clone)]
pub struct SendPacket<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub packet: T,
    pub client_id: u64,
}

#[derive(Debug, Event, Clone)]
pub struct ReceivePacket<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub packet: T,
    pub client_id: u64,
}
