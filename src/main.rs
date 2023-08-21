use bytebuffer::ByteBuffer;
use env_logger;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::{error, info};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{collections::HashMap, env, fmt::Display, hash::Hash, io::Error, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, RwLock},
};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9000".to_string());
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    let mut next_socket_id: u32 = 0;
    let world = Arc::new(RwLock::new(World::new()));

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, next_socket_id, world.clone()));
        next_socket_id += 1;
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream, socket_id: u32, world: Arc<RwLock<World>>) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    info!(
        "New WebSocket connection: {} Socket ID: {}",
        addr, socket_id
    );

    let (mut write, mut read) = ws_stream.split();

    {
        // Insert a new player for this socket
        let mut world = world.write().await;
        world.entities.insert(socket_id, Entity::new(socket_id));
    }

    loop {
        match read.next().await {
            Some(msg) => match msg {
                Ok(msg) => {
                    let result = process_message(socket_id, world.clone(), msg, &mut write).await;
                    if let Err(e) = result {
                        error!("Error processing message: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    error!("Error reading WebSocket message: {}", e);
                    break;
                }
            },
            None => break,
        }
    }

    {
        // Remove the player for this socket
        let mut world = world.write().await;
        world.entities.remove(&socket_id);
    }
}

async fn process_message(
    socket_id: u32,
    world: Arc<RwLock<World>>,
    message: Message,
    write: &mut WriteSource,
) -> Result<(), MessageError> {
    info!("Got message: {}", message);
    let mut buffer = ByteBuffer::from_vec(message.into_data());
    let packet_id = PacketID::try_from_primitive(
        buffer
            .read_u8()
            .map_err(|_| MessageError::UnprocessableInput("Could not read u8.".to_owned()))?,
    )
    .map_err(|_| MessageError::UnprocessableInput("Could not cast u8 to PacketID.".to_owned()))?;

    process_packet(socket_id, world, packet_id, buffer, write).await
}

async fn process_packet(
    socket_id: u32,
    world: Arc<RwLock<World>>,
    packet_id: PacketID,
    buffer: ByteBuffer,
    write: &mut WriteSource,
) -> Result<(), MessageError> {
    match packet_id {
        PacketID::MoveLeft => {}
        PacketID::MoveRight => {}
        PacketID::MoveUp => {}
        PacketID::MoveDown => {}
    }
    Ok(())
}

pub type ReadSource = SplitStream<WebSocketStream<TcpStream>>;
pub type WriteSource = SplitSink<WebSocketStream<TcpStream>, Message>;

#[derive(Debug)]
pub enum MessageError {
    UnprocessableInput(String),
}

impl Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnprocessableInput(msg) => write!(f, "UnprocessableInput: {}", msg),
        }
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum PacketID {
    MoveLeft = 1,
    MoveRight = 2,
    MoveUp = 3,
    MoveDown = 4,
}

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn zero() -> Self {
        Vec2::new(0f32, 0f32)
    }
}

pub struct Player<'a> {
    pub entity: &'a Entity,
    pub connection_read: ReadSource,
    pub connection_write: Mutex<WriteSource>,
}

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

pub struct World<'a> {
    pub players: HashMap<u32, Player<'a>>,
    pub entities: HashMap<u32, Entity>,
    pub next_free_entity_id: u32,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        World {
            players: HashMap::<u32, Player>::new(),
            entities: HashMap::<u32, Entity>::new(),
            next_free_entity_id: 0,
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let new_entity = Entity::new(self.next_free_entity_id);
        self.entities.insert(self.next_free_entity_id, new_entity);
        self.next_free_entity_id += 1;
        new_entity
    }

    pub fn remove_entity(&mut self, id: u32) -> Option<Entity> {
        self.entities.remove(&id)
    }

    pub fn add_player(player: Player<'a>) {
        player.entity = create
    }
}
