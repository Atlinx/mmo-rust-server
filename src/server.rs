use env_logger;
use log::{error, info};
use tokio::{net::TcpListener, task::JoinHandle};

use crate::{
    client::Client,
    types::ArcRwLock,
    world_manager::{WorldManager, WorldManagerConfig},
};

#[derive(Clone)]
pub struct ServerConfig {
    pub log_level: log::LevelFilter,
    pub address: String,
    pub world_manager_config: WorldManagerConfig,
}

impl ServerConfig {
    pub fn default() -> ServerConfig {
        ServerConfig {
            log_level: log::LevelFilter::Debug,
            address: "127.0.0.1:9000".to_string(),
            world_manager_config: WorldManagerConfig::default(),
        }
    }
}

pub struct Server {
    pub config: ServerConfig,
    pub listener: Option<TcpListener>,

    pub world_manager: ArcRwLock<WorldManager>,
}

impl Server {
    pub fn default() -> Server {
        Self::new(ServerConfig::default())
    }

    pub fn new(config: ServerConfig) -> Server {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();

        let world_manager = ArcRwLock::new(WorldManager::new(config.world_manager_config.clone()));

        Server {
            config,
            listener: None,
            world_manager,
        }
    }

    pub async fn start(&mut self) {
        let try_socket = TcpListener::bind(&self.config.address).await;
        self.listener = Some(try_socket.expect("Failed to bind listener"));
        info!("Server listening on: {}", &self.config.address);

        while let Ok((stream, _)) = self.listener.as_ref().unwrap().accept().await {
            let client = Client::from_stream(stream).await;
            let mut writeable_world_manager = self.world_manager.write().await;
            match writeable_world_manager.try_connect_client(client) {
                Err(e) => {
                    error!("Could not connect client to world: {}", e);
                }
                Ok(id) => {
                    let client_arc = writeable_world_manager.clients.get(&id).unwrap();
                    tokio::spawn(start_client_process(client_arc.clone()));
                }
            }
        }
    }
}

async fn start_client_process(client: ArcRwLock<Client>) {
    loop {
        let mut writable_client = client.write().await;
        if !writable_client.process_next().await {
            break;
        }
    }
}
