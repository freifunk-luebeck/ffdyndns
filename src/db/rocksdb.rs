use chrono::{Utc, DateTime};
use crate::domain::Dname;
use crate::ffdyndns::Token;
use crate::sha256;
use log::{info, warn, error};
use rocksdb;
use serde_json as json;
use serde::{Serialize, Deserialize};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use std::sync::{Mutex, Arc};
use super::Database;


#[derive(Clone)]
pub struct Rocksdb {
	conn: Arc<Mutex<rocksdb::DB>>,
}

impl Rocksdb {
    fn new(path: PathBuf) -> Self {
		let conn = rocksdb::DB::open_default(path).unwrap();
		Self { conn: Arc::new(Mutex::new(conn)) }
	}
}


impl Database for Rocksdb {
    fn get(&self, key: String) -> Option<Vec<u8>> {
        self.conn.lock().unwrap().get(key.as_bytes()).unwrap()
    }

	fn set(&self, key: String, val: Vec<u8>) -> Result<(),()> {
        self.conn.lock().unwrap().put(
            key.as_bytes(),
            val
        ).map_err(|_| ())
    }

	fn delete(&self, key: String) {
        self.conn.lock().unwrap().delete(key.as_bytes());
    }


}
