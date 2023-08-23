use crate::types::Vec2;

pub struct Entity {
    pub id: u32,
    pub pos: Vec2,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Entity {
            id,
            pos: Vec2::zero(),
        }
    }
}
