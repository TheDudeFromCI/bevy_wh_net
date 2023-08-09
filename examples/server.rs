use bevy::prelude::*;
use bevy_wh_net::server::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum PingPongPacket {
    Ping,
    Pong,
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(ServerNetworkingPlugin::<PingPongPacket> {
            ip:          String::from("127.0.0.1:8123"),
            max_clients: 16,
            _phantom:    Default::default(),
        })
        .add_systems(Update, (client_connected, client_disconnected, pong_pong))
        .run();
}

fn client_connected(mut events: EventReader<ClientConnectedEvent>) {
    for ev in events.iter() {
        println!("Client {} has joined the server.", ev.client_id);
    }
}

fn client_disconnected(mut events: EventReader<ClientDisconnectedEvent>) {
    for ev in events.iter() {
        println!("Client {} has left the server.", ev.client_id);
    }
}

fn pong_pong(
    mut ping_event: EventReader<ReceivePacket<PingPongPacket>>,
    mut pong_event: EventWriter<SendPacket<PingPongPacket>>,
) {
    for ev in ping_event.iter() {
        pong_event.send(SendPacket {
            packet:    PingPongPacket::Pong,
            client_id: ev.client_id,
        });

        println!("PingPong! from {}", ev.client_id);
    }
}
