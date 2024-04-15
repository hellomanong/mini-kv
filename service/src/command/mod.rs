use std::sync::Arc;

use crate::{
    error::KvError,
    storage::{memory::MemTable, Storage},
};
use protocol::pb::abi::{command_request::RequestData, CommandRequest, CommandResponse};
use tracing::{debug, info};

mod command;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

pub struct ServiceInner<Store> {
    store: Store,
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        info!("Got request: {:?}", cmd);
        let res = dispatch(cmd, &self.inner.store);
        info!("Executed response: {:?}", res);
        res
    }
}

fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(v)) => v.execute(store),
        Some(RequestData::Hset(v)) => v.execute(store),
        Some(RequestData::Hgetall(v)) => v.execute(store),
        None => KvError::InvaliadCommand("Request has no data".into()).into(),
        _ => KvError::Internal("Not implemented".into()).into(),
    }
}
