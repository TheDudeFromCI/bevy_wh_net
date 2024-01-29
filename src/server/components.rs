use std::fmt;

use bevy::prelude::*;
use bevy_renet::renet::ClientId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionState {
    /// The client is connected, but has not yet joined the game. They are
    /// still waiting to be authenticated.
    Connected,

    /// The client has joined the game and is now playing.
    Joined,

    /// The client has been disconnected from the game.
    Disconnected,
}

impl fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionState::Connected => write!(f, "Connected"),
            ConnectionState::Joined => write!(f, "Joined"),
            ConnectionState::Disconnected => write!(f, "Disconnected"),
        }
    }
}

#[derive(Debug, Component)]
pub struct ClientConnection {
    client_id: ClientId,
    username: String,
    connection_state: ConnectionState,
}

impl ClientConnection {
    pub fn new(client_id: ClientId, username: String) -> Self {
        Self {
            client_id,
            username,
            connection_state: ConnectionState::Connected,
        }
    }

    pub fn client_id(&self) -> ClientId {
        self.client_id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn connection_state(&self) -> ConnectionState {
        self.connection_state
    }

    pub(super) fn disconnect(&mut self) {
        self.connection_state = ConnectionState::Disconnected;
    }

    pub(super) fn join(&mut self) {
        self.connection_state = ConnectionState::Joined;
    }
}

impl fmt::Display for ClientConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}(id: {}, {})",
            self.username, self.client_id, self.connection_state
        )
    }
}
