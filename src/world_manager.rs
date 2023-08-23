use std::{collections::HashMap, fmt::Display};

use crate::{client::Client, types::ArcRwLock, world::World};

#[derive(Clone)]
pub struct WorldManagerConfig {
    pub max_client_count: usize,
}

impl WorldManagerConfig {
    pub fn default() -> WorldManagerConfig {
        WorldManagerConfig {
            max_client_count: 100,
        }
    }
}

pub struct WorldManager {
    pub config: WorldManagerConfig,

    pub next_client_id: usize,
    pub clients: HashMap<usize, ArcRwLock<Client>>,

    pub next_world_id: usize,
    pub worlds: HashMap<usize, ArcRwLock<World>>,
}

impl WorldManager {
    pub fn default() -> WorldManager {
        Self::new(WorldManagerConfig::default())
    }

    pub fn new(config: WorldManagerConfig) -> WorldManager {
        WorldManager {
            config,
            next_client_id: 0,
            clients: HashMap::new(),
            next_world_id: 0,
            worlds: HashMap::new(),
        }
    }

    pub fn try_connect_client(&mut self, mut client: Client) -> Result<usize, WorldError> {
        if self.clients.len() >= self.config.max_client_count {
            return Err(WorldError::TooManyClients);
        }

        client.id = self.next_client_id;
        let client_id = client.id;
        self.clients.insert(client.id, ArcRwLock::new(client));
        self.next_client_id += 1;

        return Ok(client_id);
    }

    pub fn disconnect_client(&mut self, client_id: usize) -> bool {
        self.clients.remove(&client_id).is_some()
    }

    pub fn add_world(&mut self, mut world: World) -> usize {
        world.id = self.next_world_id;
        let world_id = world.id;
        self.worlds.insert(world.id, ArcRwLock::new(world));
        self.next_world_id += 1;

        return world_id;
    }

    pub fn remove_world(&mut self, world_id: usize) -> bool {
        self.worlds.remove(&world_id).is_some()
    }
}

pub enum WorldError {
    TooManyClients,
}

impl Display for WorldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyClients => write!(f, "TooManyClients"),
        }
    }
}
