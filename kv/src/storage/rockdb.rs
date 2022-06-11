use std::{convert::TryInto, sync::Arc};

use crate::{Kvpair, Storage, StorageIter, Value};
use rocksdb::{BoundColumnFamily, ColumnFamilyDescriptor, Options, DB};

pub struct RocksDb(DB);

impl RocksDb {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Self {
        let mut cf_opts = Options::default();
        cf_opts.set_max_write_buffer_number(16);
        let cf = ColumnFamilyDescriptor::new("table1", cf_opts);

        let mut db_opts = Options::default();
        db_opts.create_missing_column_families(true);
        db_opts.create_if_missing(true);

        let db = DB::open_cf_descriptors(&db_opts, path, vec![cf]).unwrap();
        Self(db)
    }

    pub fn get_or_create_table(&self, name: &str) -> Arc<BoundColumnFamily> {
        if self.0.cf_handle(name).is_none() {
            self.0.create_cf(name, &Options::default()).unwrap();
        }
        self.0.cf_handle(name).unwrap()
    }
}

impl Storage for RocksDb {
    fn get(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
        let cf = self.get_or_create_table(table);
        self.0
            .get_cf(&cf, key)?
            .map(|v| (&v[..]).try_into())
            .transpose()
    }

    fn set(
        &self,
        table: &str,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Option<crate::Value>, crate::KvError> {
        let cf = self.get_or_create_table(table);
        let key = key.into();
        let old: Option<Result<crate::Value, _>> =
            self.0.get_cf(&cf, &key)?.map(|v| (&v[..]).try_into());
        let val: Vec<u8> = value.into().try_into()?;
        let val: &[u8] = val.as_ref();
        self.0.put_cf(&cf, &key, val).unwrap();
        old.transpose()
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, crate::KvError> {
        let cf = self.get_or_create_table(table);
        Ok(self.0.key_may_exist_cf(&cf, key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
        let cf = self.get_or_create_table(table);
        let old: Option<Result<crate::Value, _>> = self
            .0
            .get_cf(&cf, key.as_bytes())?
            .map(|v| (&v[..]).try_into());
        self.0.delete_cf(&cf, key)?;
        old.transpose()
    }

    fn get_all(&self, table: &str) -> Result<Vec<crate::Kvpair>, crate::KvError> {
        let cf = self.get_or_create_table(table);
        Ok(self
            .0
            .iterator_cf(&cf, rocksdb::IteratorMode::Start)
            .map(|v| v.into())
            .collect())
    }

    fn get_iter<'a>(
        &'a self,
        table: &str,
    ) -> Result<Box<dyn Iterator<Item = crate::Kvpair> + 'a>, crate::KvError> {
        let cf = self.get_or_create_table(table);
        let iter = self.0.iterator_cf(&cf, rocksdb::IteratorMode::Start);
        let iter = StorageIter::new(iter);
        Ok(Box::new(iter))
    }
}

impl From<(std::boxed::Box<[u8]>, std::boxed::Box<[u8]>)> for Kvpair {
    fn from(value: (std::boxed::Box<[u8]>, std::boxed::Box<[u8]>)) -> Self {
        Kvpair::new(
            std::str::from_utf8(value.0.as_ref()).unwrap(),
            value.1.as_ref().try_into().unwrap(),
        )
    }
}
