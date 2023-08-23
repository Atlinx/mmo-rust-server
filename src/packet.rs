use async_trait::async_trait;
use bytebuffer::ByteBuffer;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::client::{Client, MessageError};

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum PacketID {
    PlayerMove = 1,
    PlayerShoot = 2,
}

#[async_trait]
pub trait PacketHandler {
    fn can_handle(&self, packet_id: PacketID) -> bool;
    async fn process_packet(
        &self,
        client: &mut Client,
        buffer: ByteBuffer,
    ) -> Result<(), MessageError>;
}
