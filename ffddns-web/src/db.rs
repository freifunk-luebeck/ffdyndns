use rusqlite as sqlite;
use sqlite::params;
use std::path::PathBuf;
use chrono::{Utc, DateTime};
use std::net::{Ipv4Addr, Ipv6Addr};
use log::{info};
use std::sync::{Mutex, Arc};
use rocksdb;


pub struct Database {
	conn: Arc<Mutex<sqlite::Connection>>,
}


unsafe impl Send for Database {}
unsafe impl Sync for Database {}


impl Database {
	pub fn new(path: PathBuf) -> Self {
		let conn: sqlite::Connection = sqlite::Connection::open(&path).unwrap();
		conn.execute_batch(include_str!("init.sql")).unwrap();
		conn.pragma(None, "synchronous", &"OFF".to_string(), |_| Ok(())).unwrap();
		Database { conn: Arc::new(Mutex::new(conn)) }
	}

	pub fn get_all_domains(&self) -> Vec<Domain> {
		let db = self.conn.lock().unwrap();
		let mut stmt: sqlite::Statement = db.prepare("SELECT * FROM domains").unwrap();

		stmt.query_map(
			params![],
			|row| Ok(Domain::from_row(row))
		).unwrap().map(|x| x.unwrap()).collect()
	}

	pub fn insert_new_domain(&self, d: &Domain) {
		self.conn.lock().unwrap().execute(
			"INSERT INTO domains VALUES ($1, $2, $3, $4, $5)",
			params![d.domainname, d.token, d.lastupdate, d.ipv4.map(|x| x.to_string()), d.ipv6.map(|x| x.to_string())]
		).unwrap();
	}


	pub fn get_domain(&self, domain: &String) -> Option<Domain> {
		let r: sqlite::Result<_> = self.conn.lock().unwrap().query_row_and_then(
			"SELECT * FROM domains WHERE domainname=$1",
			params![domain],
			|row| Ok(Domain::from_row(row))
		);

		match r {
			Err(_) => None,
			Ok(o) => Some(o)
		}
	}

	pub fn remove_domain(&self, d: String) {
		self.conn.lock().unwrap().execute(
			"DELETE FROM domains WHERE domainname=$1",
			params![d]
		).unwrap();
	}


	pub fn update_lastupdate(&self, d: &String, lastupdate: DateTime<Utc>) {
		self.conn.lock().unwrap().execute(
			"UPDATE domains SET lastupdate=$2 WHERE domainname=$1",
			params![d, lastupdate]
		).unwrap();
	}

	pub fn update_ipv4(&self, d: &String, addr: Ipv4Addr) {
		info!("updating {}", d);
		self.conn.lock().unwrap().execute(
			"UPDATE domains SET ipv4=$2 WHERE domainname=$1",
			params![d, addr.to_string()]
		).unwrap();
	}

	pub fn update_ipv6(&self, d: &String, addr: Ipv6Addr) {
		self.conn.lock().unwrap().execute(
			"UPDATE domains SET ipv6=$2 WHERE domainname=$1",
			params![d, addr.to_string()]
		).unwrap();
	}
}


#[derive(Debug, Clone)]
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
