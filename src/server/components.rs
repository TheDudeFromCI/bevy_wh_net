use bevy::prelude::*;
use bevy_renet::renet::ClientId;

#[derive(Debug, Component)]
pub struct ClientConnection {
    client_id: ClientId,
    connected: bool,
}

impl ClientConnection {
    pub fn new(client_id: ClientId) -> Self {
        Self {
            client_id,
            connected: true,
        }
    }

    pub fn client_id(&self) -> ClientId {
        self.client_id
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub(super) fn disconnect(&mut self) {
        self.connected = false;
    }
}
