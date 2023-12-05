use bevy::prelude::*;

use crate::common::PacketContainer;

#[derive(Debug, Event)]
pub struct OnConnectToServer;

#[derive(Debug, Event)]
pub struct OnDisconnectFromServer;

#[derive(Debug, Event)]
pub struct OnCouldNotConnectToServer;

#[derive(Debug, Event, Deref)]
pub struct DoSendPacketToServer(pub PacketContainer);

#[derive(Debug, Event, Deref)]
pub struct OnReceivePacketFromServer(pub PacketContainer);

#[derive(Debug, Event)]
pub struct DoConnectToServer {
    pub ip: String,
}

#[derive(Debug, Event)]
pub struct DoDisconnectFromServer;
