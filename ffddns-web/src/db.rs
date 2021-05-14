use rusqlite as sqlite;
use sqlite::params;
use std::path::PathBuf;
use chrono::{Utc, DateTime};
use std::net::{Ipv4Addr, Ipv6Addr};
use log::{info};
use std::sync::{Mutex, Arc};
use rocksdb;
use serde::{Serialize, Deserialize};
use serde_json as json;


pub struct Database {
	conn: Arc<Mutex<rocksdb::DB>>,
}


// unsafe impl Send for Database {}
// unsafe impl Sync for Database {}


impl Database {
	pub fn new(path: PathBuf) -> Self {
		let conn = rocksdb::DB::open_default(path).unwrap();
		Database { conn: Arc::new(Mutex::new(conn)) }
	}

	// pub fn get_all_domains(&self) -> Vec<Domain> {
	// 	let db = self.conn.lock().unwrap();
	// 	let mut stmt: sqlite::Statement = db.prepare("SELECT * FROM domains").unwrap();

	// 	stmt.query_map(
	// 		params![],
	// 		|row| Ok(Domain::from_row(row))
	// 	).unwrap().map(|x| x.unwrap()).collect()
	// }

	pub fn insert_new_domain(&self, d: &Domain) {
		self.conn.lock().unwrap().put(
			&d.domainname,
			json::to_vec(&d).unwrap()
		).unwrap();
	}


	pub fn get_domain(&self, domain: &String) -> Option<Domain> {
		let r = self.conn.lock().unwrap().get(domain).unwrap();
		r.map(|x| json::from_slice(&x).unwrap())
	}

	pub fn remove_domain(&self, domain: String) {
		self.conn.lock().unwrap().delete(domain).unwrap();
	}


	pub fn update_lastupdate(&self, domain: &String, lastupdate: DateTime<Utc>) {
		let mut d = self.get_domain(domain).unwrap();
		d.lastupdate = Some(lastupdate);

		self.conn.lock().unwrap().put(
			domain,
			json::to_vec(&d).unwrap()
		).unwrap();
	}

	pub fn update_ipv4(&self, domain: &String, addr: Ipv4Addr) {
		let mut d = self.get_domain(domain).unwrap();
		d.ipv4 = Some(addr);

		self.conn.lock().unwrap().put(
			domain,
			json::to_vec(&d).unwrap()
		).unwrap();
	}

	pub fn update_ipv6(&self, domain: &String, addr: Ipv6Addr) {
		let mut d = self.get_domain(domain).unwrap();
		d.ipv6 = Some(addr);

		self.conn.lock().unwrap().put(
			domain,
			json::to_vec(&d).unwrap()
		).unwrap();
	}
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Domain {
	pub domainname: String,
	pub token: String,
	pub lastupdate: Option<DateTime<Utc>>,
	pub ipv4: Option<Ipv4Addr>,
	pub ipv6: Option<Ipv6Addr>,
}

impl Domain {
	fn from_row(row: &sqlite::Row) -> Self {
		Self {
			domainname: row.get("domainname").unwrap(),
			token: row.get("token").unwrap(),
			lastupdate: row.get("lastupdate").unwrap(),
			ipv4: row.get::<_, Option<String>>("ipv4").unwrap().map(|x| x.parse().unwrap()),
			ipv6: row.get::<_, Option<String>>("ipv6").unwrap().map(|x| x.parse().unwrap()),
		}
	}
}

impl Domain {
	pub fn new_with_token(domain: String, token: String) -> Self {
		Self {
			domainname: domain,
			token: token,
			lastupdate: None,
			ipv4: None,
			ipv6: None
		}
	}

	/// creates a new Domain object and generates a random token
	pub fn new(domain: String) -> Self {
		Self {
			domainname: domain,
			token: generate_token(),
			lastupdate: None,
			ipv4: None,
			ipv6: None
		}
	}
}


pub fn generate_token() -> String {
	let mut token = String::new();
	for _ in 0..8 {
		token.push_str(&format!("{:02x}", rand::random::<u8>()));
	}
	token
}
