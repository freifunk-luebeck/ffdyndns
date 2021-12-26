use chrono::Utc;
use crate::Database;
use crate::db::Domain;
use crate::domain::Dname;
#[allow(unused_imports)]
use log::{info, error, warn};
use serde::{Serialize};
use std::fmt::{self, Display};
use std::net::IpAddr;
use crate::CONFIG;
use std::sync::mpsc;
use crate::nsupdate::{self, nsupdate::UpdateMessage};
use std::sync::{Arc, Mutex};
use chrono::Duration;

/// token length in bytes
/// The hex length will be double the length
const TOKEN_LENGTH: usize = 8;

pub type Token = String;

pub struct UpdateRequest {
	pub domain: String,
	pub addr: IpAddr,
	pub token: String,
}


#[derive(Clone, Debug, Serialize)]
pub enum Error {
	UpdateError(String),
	DomainNotFound,
	InvalidToken,
	InvalidDomain,
	DomainExists,
	RecordTypeNotMatching,
}

impl From<String> for Error {
	fn from(foo: String) -> Self {
		Self::UpdateError(foo)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Self::InvalidToken => "the provided token is invalid",
			Self::DomainNotFound => "the domain was not found",
			Self::UpdateError(s) => &s,
			_ => "unknown or undocumented error"
		})
	}
}


#[derive(Clone)]
pub struct Service {
	db: Database,
	updater: Arc<Mutex<mpsc::Sender<UpdateMessage>>>,
}

impl Service {
	pub fn new(db: Database) -> Self {
		Self{
			db,
			updater: Arc::new(Mutex::new(nsupdate::start_nsupdater())),
		}
	}

	pub fn update_domain(&self, update: UpdateRequest) -> Result<(), Error> {
		let db = &self.db;

		if !db.exists(&update.domain) {
			return Err(Error::DomainNotFound);
		}

		let d = db.get_domain(&update.domain).unwrap();
		info!("{:#?}", d);

		if d.token != update.token {
			return Err(Error::InvalidToken);
		}

		info!("updating ip for {} to {:?}", update.domain, update.addr);
		match update.addr {
			IpAddr::V4(addr) => db.update_ipv4(&update.domain, addr),
			IpAddr::V6(addr) => db.update_ipv6(&update.domain, addr),
		}

		db.update_lastupdate(&update.domain, Utc::now());

		self.updater
			.lock()
			.unwrap()
			.send(UpdateMessage::from_updaterequest(update))
			.unwrap();

		Ok(())
	}

	pub fn new_domain(&self, d: Dname) -> Result<Token, Error> {

		if CONFIG.get_domain_config(&d.strip_subdomain()).is_none() {
			error!("domain suffix not configured: {}", d);
			return Err(Error::InvalidDomain);
		}

		if self.db.exists(&d.to_string()) {
			return Err(Error::DomainExists);
		}

		let token = generate_token();
		let domain = Domain::new_with_token(&d, token.clone(), Duration::days(CONFIG.get_domain_config(&d.strip_subdomain()).unwrap().validity as i64));
		self.db.insert_new_domain(&domain);

		Ok(token)
	}

	pub fn clean_domains(&self) {
		let domains = self.db.get_all();

		for d in domains {
			if d.valid_until < Utc::now() {
				self.db.remove_domain(&d.domainname);

				self.updater
					.lock()
					.unwrap()
					.send(UpdateMessage::new_remove_message(d.domainname))
					.unwrap();
			}
		}
	}
}



pub fn generate_token() -> Token {
	let mut token = String::new();
	for _ in 0..TOKEN_LENGTH {
		token.push_str(&format!("{:02x}", rand::random::<u8>()));
	}
	token
}

pub fn domainname_is_blocklisted(d: &String) -> bool {
	unimplemented!()
	// todo
}
