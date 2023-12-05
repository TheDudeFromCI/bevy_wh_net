mod events;
mod systems;

use std::marker::PhantomData;
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, RenetServer};
use bevy_renet::transport::NetcodeServerPlugin;
use bevy_renet::RenetServerPlugin;
pub use events::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub const PROTOCOL_ID: u64 = 0;

#[derive(Debug)]
pub struct ServerNetworkingPlugin<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub ip: String,
    pub max_clients: usize,
    pub _phantom: PhantomData<T>,
}

impl<T> Plugin for ServerNetworkingPlugin<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        let server = RenetServer::new(ConnectionConfig::default());
        let addr = self.ip.to_socket_addrs().unwrap().next().unwrap();
        let socket = UdpSocket::bind(addr).unwrap();
        let config = ServerConfig {
            max_clients: self.max_clients,
            protocol_id: PROTOCOL_ID,
            public_addr: addr,
            authentication: ServerAuthentication::Unsecure,
        };
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let transport = NetcodeServerTransport::new(time, config, socket).unwrap();

        app.insert_resource(server)
            .insert_resource(transport)
            .add_event::<ClientConnectedEvent>()
            .add_event::<ClientDisconnectedEvent>()
            .add_event::<SendPacket<T>>()
            .add_event::<ReceivePacket<T>>()
            .add_plugins((RenetServerPlugin, NetcodeServerPlugin))
            .add_systems(
                Update,
                (
                    systems::server_event_handler,
                    systems::error_handling,
                    systems::send_packet::<T>,
                    systems::receive_packets::<T>,
                ),
            );
    }
}
