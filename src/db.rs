use crate::domain::Dname;
use std::path::PathBuf;
use chrono::{Utc, DateTime, Duration};
use std::net::{Ipv4Addr, Ipv6Addr};
#[allow(unused_imports)]
use log::{info, warn, error};
use std::sync::{Mutex, Arc};
use serde::{Serialize, Deserialize};
use serde_json as json;
use crate::ffdyndns::Token;
use crate::sha256;


#[derive(Clone)]
pub struct Database {
	conn: Arc<Mutex<sled::Db>>,
}



impl Database {
	pub fn new(path: PathBuf) -> Self {
		let conn = sled::open(path).unwrap();
		Self { conn: Arc::new(Mutex::new(conn)) }
	}

	// basic CRUD methods
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
        self.conn.lock().unwrap().remove(key).expect("cannot remove key");
    }

    fn list(&self) -> Vec<Vec<u8>> {
        self.conn.lock().unwrap()
			.iter()
            .map(|r| {
                r.unwrap().1.as_ref().to_vec()
            }).collect()
    }



	pub fn insert_new_domain(&self, d: &Domain) {
		self.set(
			sha256!(&d.domainname),
			json::to_vec(&d).unwrap()
		).unwrap();
	}


	pub fn get_domain(&self, domain: &String) -> Option<Domain> {
		let r = self.get(sha256!(domain));
		r.map(|x| json::from_slice(&x).unwrap())
	}

	pub fn remove_domain(&self, domain: &String) {
		self.delete(sha256!(domain));
	}


	pub fn update_lastupdate(&self, domain: &String, lastupdate: DateTime<Utc>) {
		let mut d = self.get_domain(domain).unwrap();
		d.lastupdate = lastupdate;

		self.set(
			sha256!(domain),
			json::to_vec(&d).unwrap()
		).unwrap();
	}

	#[allow(dead_code)]
	pub fn update_validity(&self, domain: &String, valid_until: DateTime<Utc>) {
		let mut d = self.get_domain(domain).unwrap();
		d.valid_until = valid_until;

		self.set(
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

		self.set(
			sha256!(domain),
			json::to_vec(&d).unwrap()
		).unwrap();
	}

	pub fn update_ipv6(&self, domain: &String, addr: Ipv6Addr) {
		let mut d = self.get_domain(domain).unwrap();
		d.ipv6 = Some(addr);

		self.set(
			sha256!(domain),
			json::to_vec(&d).unwrap()
		).unwrap();
	}

	pub fn exists(&self, d: &String) -> bool {
		self.get_domain(d).is_some()
	}

	pub fn get_all(&self) -> Vec<Domain> {
		self.list().iter().map(|v| {
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
	#[allow(dead_code)]
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
