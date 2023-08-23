use bytebuffer::ByteBuffer;
use futures_util::StreamExt;
use log::{error, info};
use num_enum::TryFromPrimitive;
use std::{error, fmt::Display};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{Error, Message},
    WebSocketStream,
};

use crate::{
    packet::PacketID, packet_handlers, player::Player, types::ArcRwLock,
    world_manager::WorldManager,
};

pub struct Client {
    pub id: usize,
    pub ws_stream: WebSocketStream<TcpStream>,

    pub player: Option<ArcRwLock<Player>>,
    pub world_manager: Option<ArcRwLock<WorldManager>>,
}

impl Client {
    pub async fn from_stream(stream: TcpStream) -> Client {
        let addr = stream
            .peer_addr()
            .expect("Connected streams should have a peer address");
        info!("Client address: {}", addr);

        let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Error during the websocket handshake occurred");
        info!("New Client connection: {}", addr);

        Client {
            id: 0,
            ws_stream,
            world_manager: None,
            player: None,
        }
    }

    pub async fn disconnect(&mut self) -> Result<(), Error> {
        self.ws_stream.close(None).await?;
        if let Some(world_manager) = self.world_manager.as_ref() {
            world_manager.write().await.disconnect_client(self.id);
        }
        Ok(())
    }

    pub async fn process_next(&mut self) -> bool {
        match self.ws_stream.next().await {
            Some(msg) => match msg {
                Ok(msg) => {
                    let result = self.process_message(msg).await;
                    if let Err(e) = result {
                        error!("Error processing message: {}", e);
                        return false;
                    }
                    true
                }
                Err(e) => {
                    error!("Error reading WebSocket message: {}", e);
                    false
                }
            },
            None => false,
        }
    }

    async fn process_message(&mut self, message: Message) -> Result<(), MessageError> {
        info!("Got message: {}", message);
        let mut buffer = ByteBuffer::from_vec(message.into_data());
        buffer.set_endian(bytebuffer::Endian::LittleEndian);
        let packet_id = PacketID::try_from_primitive(
            buffer
                .read_u8()
                .map_err(|_| MessageError::UnprocessableInput("Could not read u8.".to_owned()))?,
        )
        .map_err(|_| {
            MessageError::UnprocessableInput("Could not cast u8 to PacketID.".to_owned())
        })?;

        self.process_packet(packet_id, buffer).await;

        Ok(())
    }

    async fn process_packet(&mut self, packet_id: PacketID, buffer: ByteBuffer) {
        for handler in packet_handlers::PACKET_HANDLERS.iter() {
            if handler.can_handle(packet_id) {
                if let Err(e) = handler.process_packet(self, buffer).await {
                    error!("Error procesing packet: {}", e);
                }
                return;
            }
        }
    }
}

#[derive(Debug)]
pub enum MessageError {
    UnprocessableInput(String),
    UnprocessablePacket,
}

impl Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnprocessableInput(msg) => write!(f, "UnprocessableInput: {}", msg),
            Self::UnprocessablePacket => write!(f, "UnprocessablePacket"),
        }
    }
}
