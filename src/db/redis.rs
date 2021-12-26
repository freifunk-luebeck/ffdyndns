use redis;
use redis::FromRedisValue;
use redis::ToRedisArgs;
use log::info;
use redis::Commands;
use redis::RedisError;
use redis::RedisResult;
use std::path::PathBuf;
use serde_json as json;
use super::Database;
use crate::db::Domain;
use crate::sha256;


pub struct Redisdb {
    conn: redis::Connection,
}

impl Redisdb {
    pub fn new(path: &PathBuf) -> Result<Self, RedisError> {
        let client = redis::Client::open("redis://127.0.0.1/")?; // TODO extract path
        let c = client.get_connection()?;
        Ok(Self { conn: c }) // TODO Error handling
    }
}


impl Database for Redisdb {
    fn insert_new_domain(&mut self, d: &Domain) {
        let value = json::to_vec(&d).unwrap();
        let key = sha256!(&d.domainname);
        self.conn.set::<String, Vec<u8>, Vec<u8>>(key, value);

        // TODO Error handling
    }
}
