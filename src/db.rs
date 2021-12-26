use crate::domain::Dname;
use std::path::PathBuf;
use chrono::{Utc, DateTime, Duration};
use std::net::{Ipv4Addr, Ipv6Addr};
use log::{info, warn, error};
use std::sync::{Mutex, Arc};
use rocksdb;
use serde::{Serialize, Deserialize};
use serde_json as json;
use crate::ffdyndns::Token;
use crate::sha256;
use rocksdb::IteratorMode;


#[derive(Clone)]
pub struct Database {
	conn: Arc<Mutex<rocksdb::DB>>,
}



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
			sha256!(&d.domainname),
			json::to_vec(&d).unwrap()
		).unwrap();
	}


	pub fn get_domain(&self, domain: &String) -> Option<Domain> {
		let r = self.conn.lock().unwrap().get(sha256!(domain)).unwrap();
		r.map(|x| json::from_slice(&x).unwrap())
	}

	pub fn remove_domain(&self, domain: &String) {
		self.conn.lock().unwrap().delete(domain).unwrap();
	}


	pub fn update_lastupdate(&self, domain: &String, lastupdate: DateTime<Utc>) {
		let mut d = self.get_domain(domain).unwrap();
		d.lastupdate = lastupdate;

		self.conn.lock().unwrap().put(
			sha256!(domain),
			json::to_vec(&d).unwrap()
		).unwrap();
	}

	pub fn update_validity(&self, domain: &String, valid_until: DateTime<Utc>) {
		let mut d = self.get_domain(domain).unwrap();
		d.valid_until = valid_until;

		self.conn.lock().unwrap().put(
			sha256!(domain),
			json::to_vec(&d).unwrap()
		).unwrap();
	}

	pub fn update_ipv4(&self, domain: &String, addr: Ipv4Addr) {
		if !self.exists(domain) {
			warn!("tried to update nonexistend domain: {}", domain);
			return
		}

		let mut d = self.get_domain(domain).unwrap();
		d.ipv4 = Some(addr);

		self.conn.lock().unwrap().put(
			sha256!(domain),
			json::to_vec(&d).unwrap()
		).unwrap();
	}

	pub fn update_ipv6(&self, domain: &String, addr: Ipv6Addr) {
		let mut d = self.get_domain(domain).unwrap();
		d.ipv6 = Some(addr);

		self.conn.lock().unwrap().put(
			sha256!(domain),
			json::to_vec(&d).unwrap()
		).unwrap();
	}

	pub fn exists(&self, d: &String) -> bool {
		self.get_domain(d).is_some()
	}

	pub fn get_all(&self) -> Vec<Domain> {
		self.conn
			.lock()
			.unwrap()
			.iterator(IteratorMode::Start)
			.map(|(_, v)| {
				json::from_slice(&*v).unwrap()
			})
			.collect()
	}
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Domain {
	pub domainname: String,
	pub token: Token,
	pub lastupdate: DateTime<Utc>,
	pub valid_until: DateTime<Utc>,
	pub ipv4: Option<Ipv4Addr>,
	pub ipv6: Option<Ipv6Addr>,
}

impl Domain {
	// fn from_row(row: &sqlite::Row) -> Self {
	// 	Self {
	// 		domainname: row.get("domainname").unwrap(),
	// 		token: row.get("token").unwrap(),
	// 		lastupdate: row.get("lastupdate").unwrap(),
	// 		ipv4: row.get::<_, Option<String>>("ipv4").unwrap().map(|x| x.parse().unwrap()),
	// 		ipv6: row.get::<_, Option<String>>("ipv6").unwrap().map(|x| x.parse().unwrap()),
	// 	}
	// }
}

impl Domain {
	pub fn new_with_token(domain: &Dname, token: String, validity: Duration) -> Self {
		Self {
			domainname: domain.to_string(),
			token: token,
			lastupdate: Utc::now(),
			valid_until: Utc::now() + validity,
			ipv4: None,
			ipv6: None
		}
	}

	/// creates a new Domain object and generates a random token
	pub fn new(domain: String, validity: Duration) -> Self {
		Self {
			domainname: domain,
			token: crate::ffdyndns::generate_token(),
			lastupdate: Utc::now(),
			valid_until: Utc::now() + validity,
			ipv4: None,
			ipv6: None
		}
	}
}
