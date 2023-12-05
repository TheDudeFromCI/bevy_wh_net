use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_renet::renet::transport::{NetcodeServerTransport, NetcodeTransportError};
use bevy_renet::renet::{DefaultChannel, RenetServer, ServerEvent};

use super::{
    ClientConnection,
    DoSendPacketToClient,
    OnClientConnected,
    OnClientDisconnected,
    OnReceivePacketFromClient,
};
use crate::common::PacketContainer;

pub(super) fn server_event_handler(
    mut clients: Query<(Entity, &mut ClientConnection)>,
    mut server_events: EventReader<ServerEvent>,
    mut connected_events: EventWriter<OnClientConnected>,
    mut disconnected_events: EventWriter<OnClientDisconnected>,
    mut commands: Commands,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let id = commands.spawn((ClientConnection::new(*client_id),)).id();

                connected_events.send(OnClientConnected {
                    client_id: *client_id,
                    entity: id,
                });

                info!("Client {} connected.", client_id);
            }
            ServerEvent::ClientDisconnected {
                client_id,
                reason: _,
            } => {
                let id = clients
                    .iter()
                    .find(|(_, connection)| connection.client_id() == *client_id)
                    .map(|(id, _)| id)
                    .unwrap();

                clients.get_mut(id).unwrap().1.disconnect();
                disconnected_events.send(OnClientDisconnected {
                    client_id: *client_id,
                    entity: id,
                });

                info!("Client {} disconnected.", client_id);
            }
        };
    }
}

pub(super) fn send_packet(
    mut server: ResMut<RenetServer>,
    mut events: EventReader<DoSendPacketToClient>,
) {
    for ev in events.read() {
        if !server.is_connected(ev.client_id) {
            continue;
        }

        let Some(message) = ev.packet.serialize() else {
            warn!("Failed to serialize packet!");
            continue;
        };

        server.send_message(ev.client_id, DefaultChannel::ReliableOrdered, message);
    }
}

pub(super) fn receive_packets(
    mut server: ResMut<RenetServer>,
    mut events: EventWriter<OnReceivePacketFromClient>,
) {
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let Some(packet) = PacketContainer::deserialize(&message) else {
                warn!("Failed to deserialize packet from {}!", client_id);
                continue;
            };

            events.send(OnReceivePacketFromClient { packet, client_id });
        }
    }
}

pub(super) fn error_handling(mut renet_error: EventReader<NetcodeTransportError>) {
    for e in renet_error.read() {
        error!("Networking Error: {}", e);
    }
}

pub(super) fn close_connections_on_exit(
    mut app_exit_evs: EventReader<AppExit>,
    mut server: ResMut<RenetServer>,
    mut transport: ResMut<NetcodeServerTransport>,
) {
    if app_exit_evs.read().next().is_none() {
        return;
    }
    server.disconnect_all();
    transport.disconnect_all(&mut server);
}
