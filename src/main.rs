pub mod client;
pub mod entity;
pub mod packet;
pub mod packet_handlers;
pub mod player;
pub mod server;
pub mod types;
pub mod world;
pub mod world_manager;

#[tokio::main]
async fn main() {
    let mut server = server::Server::default();
    server.start().await;
}
