use axum::http::StatusCode;
use protocol::pb::abi::{CommandResponse, Hget, Hgetall, Hset, Value};

use crate::error::KvError;

use super::CommandService;

impl CommandService for Hget {
    fn execute(self, store: &impl crate::storage::Storage) -> protocol::pb::abi::CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &impl crate::storage::Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(kvs) => kvs.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl crate::storage::Storage) -> CommandResponse {
        match self.pair {
            Some(v) => match store.set(&self.table, &v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => KvError::InvaliadCommand(format!("{:?}", self)).into(),
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(err: KvError) -> Self {
        let mut result = Self {
            message: err.to_string(),
            ..Default::default()
        };

        let code = match err {
            KvError::NotFound(_, _) => StatusCode::NOT_FOUND.as_u16() as u32,
            KvError::InvaliadCommand(_) => StatusCode::BAD_REQUEST.as_u16() as u32,
            _ => 0,
        };

        result.status = code;
        result
    }
}
