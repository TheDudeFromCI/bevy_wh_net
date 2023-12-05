use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_renet::renet::transport::{
    ClientAuthentication,
    NetcodeClientTransport,
    NetcodeTransportError,
};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetClient};
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::events::*;
use super::resources::*;

pub(super) fn connect_to_server(
    mut events_conn_to_server: EventReader<TryConnectToServerEvent>,
    mut events_failed_to_conn: EventWriter<CouldNotConnectToServerEvent>,
    mut next_state: ResMut<NextState<NetworkState>>,
    mut commands: Commands,
) {
    for event in events_conn_to_server.iter().take(1) {
        let Some(addr) = resolve_ip(&event.ip) else {
            events_failed_to_conn.send(CouldNotConnectToServerEvent);
            continue;
        };

        let client = RenetClient::new(ConnectionConfig::default());
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let auth = ClientAuthentication::Unsecure {
            protocol_id: super::PROTOCOL_ID,
            client_id: time.as_millis() as u64,
            server_addr: addr,
            user_data: None,
        };
        let transport = NetcodeClientTransport::new(time, auth, socket).unwrap();

        commands.insert_resource(client);
        commands.insert_resource(transport);
        next_state.set(NetworkState::Connecting);
    }
}

pub(super) fn wait_for_connection(
    transport: Res<NetcodeClientTransport>,
    mut events: EventWriter<JoinedServerEvent>,
    mut next_state: ResMut<NextState<NetworkState>>,
) {
    if transport.is_connected() {
        next_state.set(NetworkState::Connected);
        events.send(JoinedServerEvent);
    }
}

pub(super) fn close_connection(
    current_state: Res<State<NetworkState>>,
    transport: Res<NetcodeClientTransport>,
    mut failed_con_events: EventWriter<CouldNotConnectToServerEvent>,
    mut disconnected_events: EventWriter<DisconnectedFromServerEvent>,
    mut next_state: ResMut<NextState<NetworkState>>,
) {
    if transport.is_disconnected() {
        if *current_state == NetworkState::Connecting {
            failed_con_events.send(CouldNotConnectToServerEvent);
        } else {
            disconnected_events.send(DisconnectedFromServerEvent);
        }

        next_state.set(NetworkState::NotConnected);
    }
}

pub(super) fn send_packet<T>(
    mut client: ResMut<RenetClient>,
    mut events: EventReader<SendPacket<T>>,
) where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    for ev in events.iter() {
        let Ok(message) = bincode::serialize(&ev.packet) else {
            warn!("Failed to serialize packet!");
            continue;
        };

        client.send_message(DefaultChannel::ReliableOrdered, message);
    }
}

pub(super) fn receive_packets<T>(
    mut client: ResMut<RenetClient>,
    mut events: EventWriter<ReceivePacket<T>>,
) where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let Ok(packet) = bincode::deserialize(&message) else {
            warn!("Failed to deserialize packet!");
            continue;
        };

        events.send(ReceivePacket { packet });
    }
}

pub(super) fn error_handling(mut renet_error: EventReader<NetcodeTransportError>) {
    for e in renet_error.iter() {
        error!("Networking Error: {}", e);
    }
}

fn resolve_ip(ip: &str) -> Option<SocketAddr> {
    ip.to_socket_addrs().ok()?.next()
}
