use dashmap::{mapref::one::Ref, DashMap};
use protocol::pb::abi::{Kvpair, Value};

use super::Storage;

#[derive(Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, crate::error::KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.get(key).map(|v| v.value().clone()))
    }

    fn set(
        &self,
        table: &str,
        key: &str,
        value: Value,
    ) -> Result<Option<Value>, crate::error::KvError> {
        let table = self.get_or_create_table(table);
        let v = table.insert(key.into(), value);
        Ok(v)
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, crate::error::KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.contains_key(key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, crate::error::KvError> {
        let table = self.get_or_create_table(table);
        let v = table.remove(key).map(|(_k, v)| v);
        Ok(v)
    }

    fn get_all(
        &self,
        table: &str,
    ) -> Result<Vec<protocol::pb::abi::Kvpair>, crate::error::KvError> {
        let table = self.get_or_create_table(table);
        let kvpairs = table
            .iter()
            .map(|v| Kvpair::new(v.key(), v.value().clone()))
            .collect::<Vec<_>>();

        Ok(kvpairs)
    }

    fn get_iter(
        &self,
        table: &str,
    ) -> Result<Box<dyn Iterator<Item = protocol::pb::abi::Kvpair>>, crate::error::KvError> {
        let table = self.get_or_create_table(table);
        let kvpairs = table
            .iter()
            .map(|v| Kvpair::new(v.key(), v.value().clone()))
            .collect::<Vec<_>>();

        Ok(Box::new(kvpairs.into_iter()))
    }
}
