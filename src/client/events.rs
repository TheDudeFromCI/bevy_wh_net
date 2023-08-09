use bevy::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Event, Clone)]
pub struct TryConnectToServerEvent {
    pub ip: String,
}

#[derive(Debug, Event, Clone)]
pub struct JoinedServerEvent;

#[derive(Debug, Event, Clone)]
pub struct DisconnectedFromServerEvent;

#[derive(Debug, Event, Clone)]
pub struct CouldNotConnectToServerEvent;

#[derive(Debug, Event, Clone)]
pub struct SendPacket<T>
where T: Serialize + DeserializeOwned + Send + Sync + 'static {
    pub packet: T,
}

#[derive(Debug, Event, Clone)]
pub struct ReceivePacket<T>
where T: Serialize + DeserializeOwned + Send + Sync + 'static {
    pub packet: T,
}
