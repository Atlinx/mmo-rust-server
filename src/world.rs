use std::collections::HashMap;

use crate::entity::Entity;
use crate::player::Player;
use crate::types::*;

pub struct World {
    pub id: usize,

    pub players: HashMap<u32, ArcRwLock<Player>>,
    pub entities: HashMap<u32, ArcRwLock<Entity>>,
    pub next_free_entity_id: u32,
}

impl World {
    pub fn new() -> Self {
        World {
            id: 0,
            players: HashMap::<u32, ArcRwLock<Player>>::new(),
            entities: HashMap::<u32, ArcRwLock<Entity>>::new(),
            next_free_entity_id: 0,
        }
    }

    pub fn create_entity(&mut self) -> ArcRwLock<Entity> {
        let new_entity = Entity::new(self.next_free_entity_id);
        self.entities
            .insert(self.next_free_entity_id, ArcRwLock::new(new_entity));
        let inserted_entity = self.entities.get(&self.next_free_entity_id);
        self.next_free_entity_id += 1;
        inserted_entity
            .expect("Expect entity to exist after inserting")
            .clone()
    }

    pub fn remove_entity(&mut self, id: u32) -> Option<ArcRwLock<Entity>> {
        let removed_entity = self.entities.remove(&id);
        self.players.remove(&id);
        removed_entity
    }

    pub async fn add_player(&mut self) -> ArcRwLock<Player> {
        !todo!("Finish this");
        // let entity = self.create_entity();
        // let entity_id = entity.read().await.id;
        // let player = Player {
        //     entity_id: entity_id,
        //     entity: entity,
        //     client:
        // };
        // let id = player.entity.read().await.id;
        // self.players.insert(id, ArcRwLock::new(player));
        // self.players
        //     .get(&id)
        //     .expect("Expect player to exist after inserting")
        //     .clone()
    }
}
