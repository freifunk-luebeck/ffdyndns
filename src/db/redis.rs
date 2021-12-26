extern crate redis;
use redis::FromRedisValue;
use redis::ToRedisArgs;
use log::info;
use redis::Commands;
use redis::RedisError;
use redis::RedisResult;
use std::path::PathBuf;
use serde_json as json;

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
        self.conn.set::<String, Vec<u8>, RedisResult<Vec<u8>>>(key, value).unwrap();

        // TODO Error handling
    }

    // pub fn get_domain(&self, domain: &String) -> Option<Domain> {
    //     let r: Vec<u8> = self.conn.get(sha256!(domain)).unwrap(); // TODO
    //     json::from_slice(&r).unwrap()   
    // }
}
