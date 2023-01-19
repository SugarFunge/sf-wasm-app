use bevy::prelude::*;
use crossbeam::channel;
use serde::{Serialize};
use tokio::runtime::Runtime;

#[derive(Resource)]
pub struct TokioRuntime {
    pub runtime: std::sync::Arc<Runtime>,
}

#[derive(Resource, Deref, Clone)]
pub struct InputSender<T>(pub channel::Sender<T>);

#[derive(Resource, Deref, Clone)]
pub struct InputReceiver<T>(pub channel::Receiver<T>);

#[derive(Resource, Deref, Clone)]
pub struct OutputSender<T>(pub channel::Sender<T>);

#[derive(Resource, Deref, Clone)]
pub struct OutputReceiver<T>(pub channel::Receiver<T>);

pub trait Request<T: Serialize + Send + Sync>: Send + Sync + 'static {
    fn endpoint(&self) -> &str;
    fn input(&self) -> Option<T>;
}
