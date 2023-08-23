use lazy_static::lazy_static;

use self::player_move_handler::PlayerMoveHandler;
use crate::packet::*;

pub mod player_move_handler;

lazy_static! {
    pub static ref PACKET_HANDLERS: Vec<Box<dyn PacketHandler + Sync>> =
        vec![Box::new(PlayerMoveHandler())];
}
