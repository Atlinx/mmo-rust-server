use futures_util::{
    lock::Mutex,
    stream::{SplitSink, SplitStream},
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tokio::{
    net::TcpStream,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

pub type ReadSource = SplitStream<WebSocketStream<TcpStream>>;
pub type WriteSource = SplitSink<WebSocketStream<TcpStream>, Message>;

pub struct ArcMutex<T>(Arc<Mutex<T>>);
pub struct ArcRwLock<T>(Arc<RwLock<T>>);

impl<T> ArcMutex<T> {
    pub fn new(value: T) -> ArcMutex<T> {
        ArcMutex(Arc::new(Mutex::new(value)))
    }
}

impl<T> Deref for ArcMutex<T> {
    type Target = Arc<Mutex<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ArcRwLock<T> {
    pub fn new(value: T) -> ArcRwLock<T> {
        ArcRwLock(Arc::new(RwLock::new(value)))
    }
}

impl<T> Deref for ArcRwLock<T> {
    type Target = Arc<RwLock<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for ArcRwLock<T> {
    fn clone(&self) -> Self {
        ArcRwLock(self.0.clone())
    }
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
