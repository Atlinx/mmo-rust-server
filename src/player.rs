use crate::client::Client;
use crate::entity::Entity;
use crate::types::*;

pub struct Player {
    pub entity_id: u32,
    pub entity: ArcRwLock<Entity>,
    pub client: ArcRwLock<Client>,
}

impl Player {
    pub fn new() -> Player {
        todo!("Finish this")
    }
}
