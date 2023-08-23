use async_trait::async_trait;
use bytebuffer::ByteBuffer;
use log::info;

use crate::{
    client::{Client, MessageError},
    packet::*,
};

pub struct PlayerMoveHandler();

#[async_trait]
impl PacketHandler for PlayerMoveHandler {
    fn can_handle(&self, packet_id: PacketID) -> bool {
        packet_id == PacketID::PlayerMove
    }
    async fn process_packet(
        &self,
        client: &mut Client,
        buffer: ByteBuffer,
    ) -> Result<(), MessageError> {
        info!("Processed PlayerMove packet for client {}", client.id);
        Ok(())
    }
}
