use bevy::prelude::*;
use bevy_renet::renet::transport::NetcodeTransportError;
use bevy_renet::renet::{DefaultChannel, RenetServer, ServerEvent};
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::{ClientConnectedEvent, ClientDisconnectedEvent, ReceivePacket, SendPacket};

pub(super) fn server_event_handler(
    mut server_events: EventReader<ServerEvent>,
    mut connected_events: EventWriter<ClientConnectedEvent>,
    mut disconnected_events: EventWriter<ClientDisconnectedEvent>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                connected_events.send(ClientConnectedEvent {
                    client_id: *client_id,
                });
            }
            ServerEvent::ClientDisconnected {
                client_id,
                reason: _,
            } => {
                disconnected_events.send(ClientDisconnectedEvent {
                    client_id: *client_id,
                });
            }
        };
    }
}

pub(super) fn send_packet<T>(
    mut server: ResMut<RenetServer>,
    mut events: EventReader<SendPacket<T>>,
) where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    for ev in events.iter() {
        let Ok(message) = bincode::serialize(&ev.packet) else {
            warn!("Failed to serialize packet!");
            continue;
        };

        server.send_message(ev.client_id, DefaultChannel::ReliableOrdered, message);
    }
}

pub(super) fn receive_packets<T>(
    mut server: ResMut<RenetServer>,
    mut events: EventWriter<ReceivePacket<T>>,
) where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    for client_id in server.clients_id().into_iter() {
        while let Some(serialized_message) =
            server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let Ok(packet) = bincode::deserialize(&serialized_message) else {
                warn!("Failed to deserialize packet from {}!", client_id);
                continue;
            };

            events.send(ReceivePacket { packet, client_id });
        }
    }
}

pub(super) fn error_handling(mut renet_error: EventReader<NetcodeTransportError>) {
    for e in renet_error.iter() {
        error!("Networking Error: {}", e);
    }
}
