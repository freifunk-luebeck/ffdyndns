use chrono::Utc;
use crate::Database;
use crate::db::Domain;
use crate::domain::Dname;
use log::{info, error, warn};
use serde::{Serialize, Deserialize};
use std::fmt::{self, Display};
use std::net::IpAddr;
use crate::CONFIG;

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



pub struct Service {
	db: Database,
}

impl Service {
	pub fn new(db: Database) -> Self {
		Self{
			db
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

		info!("write new ip to database: {:?}", update.addr);
		match update.addr {
			IpAddr::V4(addr) => db.update_ipv4(&update.domain, addr),
			IpAddr::V6(addr) => db.update_ipv6(&update.domain, addr),
		}

		db.update_lastupdate(&update.domain, Utc::now());

		Ok(())
	}

	pub fn new_domain(&self, d: Dname) -> Result<Token, Error> {

		let in_config = &CONFIG.domain.iter()
			.map(|x| Dname::new(x.name.clone()))
			.fold(false, |acc, x| acc || x.is_subdomain(&d));

		if !in_config {
			return Err(Error::InvalidDomain);
		}

		if self.db.exists(&d.to_string()) {
			return Err(Error::DomainExists);
		}

		let token = generate_token();
		let domain = Domain::new_with_token(d, token.clone());
		self.db.insert_new_domain(&domain);

		Ok(token)
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
