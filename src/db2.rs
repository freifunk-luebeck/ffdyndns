extern crate redis;
use log::info;
use redis::Commands;
use redis::FromRedisValue;
use redis::RedisError;
use redis::RedisResult;
use redis::ToRedisArgs;
use serde_json as json;
use std::path::PathBuf;

use crate::db::Domain;
use crate::sha256;

pub struct Database {
    conn: redis::Connection,
}

impl Database {
    pub fn new(path: PathBuf) -> Result<Self, RedisError> {
        let client = redis::Client::open("redis://127.0.0.1/")?; // TODO extract path
        let c = client.get_connection()?;
        Ok(Database { conn: c }) // TODO Error handling
    }

    pub fn insert_new_domain(&mut self, d: &Domain) {
        let value = json::to_vec(&d).unwrap();
        let key = sha256!(&d.domainname);
        let _ : () = self.conn.set::<String,  Vec<u8>, _>(key, value).unwrap();  // TODO Error handling
    }

    pub fn get_domain(&mut self, domain: &String) -> Option<Domain> {
        let r: Vec<u8> = self.conn.get::<String, Vec<u8>>(sha256!(domain)).unwrap(); // TODO  Error handling
        json::from_slice(&r).unwrap()
    }
}

#[test]
fn it_saves_domain() {
    let mut db = Database::new(PathBuf::from("redis://127.0.0.1")).unwrap();
    let mut d = Domain::new(String::from("kaputt.cloud"));
    db.insert_new_domain(&mut d);
    let res = db.get_domain(&String::from("kaputt.cloud")).unwrap();
    assert_eq!(d.token, res.token);
}
