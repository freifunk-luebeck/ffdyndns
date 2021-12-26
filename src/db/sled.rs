use chrono::{Utc, DateTime};
use crate::domain::Dname;
use crate::ffdyndns::Token;
use crate::sha256;
use log::{info, warn, error};
use sled;
use serde_json as json;
use serde::{Serialize, Deserialize};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use std::sync::{Mutex, Arc};
use super::Database;



#[derive(Clone)]
pub struct Rocksdb {
	conn: Arc<Mutex<sled::Db>>,
}

impl Rocksdb {
    fn new(path: PathBuf) -> Self {
		let conn = sled::open(path).unwrap();
		Self { conn: Arc::new(Mutex::new(conn)) }
	}
}


impl Database for Rocksdb {
    fn set(&self, key: String, val: Vec<u8>) -> Result<(),()> {
        self.conn.lock().unwrap().insert(
            key.as_bytes(),
            val
        ).map_err(|_| ()).map(|_| ())
    }


    fn get(&self, key: String) -> Option<Vec<u8>> {
        self.conn.lock().unwrap().get(key.as_bytes()).unwrap().map(|x| x.as_ref().to_vec())
    }


	fn delete(&self, key: String) {
        self.conn.lock().unwrap().remove(key);
    }

    fn list(&self) -> &mut dyn Iterator<Item = Vec<u8>> {
        &mut self.conn.lock().unwrap().iter()
            .map(|r| {
                r.unwrap().1.as_ref().to_vec()
            })
    }

}
