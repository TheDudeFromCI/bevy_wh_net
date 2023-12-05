use bevy::prelude::*;

use crate::common::PacketContainer;

#[derive(Debug, Event)]
pub struct OnJoinedServer;

#[derive(Debug, Event)]
pub struct OnDisconnectedFromServer;

#[derive(Debug, Event)]
pub struct OnCouldNotConnectToServer;

#[derive(Debug, Event, Deref)]
pub struct DoSendPacket(pub PacketContainer);

#[derive(Debug, Event, Deref)]
pub struct OnReceivePacket(pub PacketContainer);

#[derive(Debug, Event)]
pub struct DoConnectToServer {
    pub ip: String,
}

#[derive(Debug, Event)]
pub struct DoDisconnectFromServer;
