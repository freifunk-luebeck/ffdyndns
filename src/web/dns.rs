use log::{debug, error, info};
use tera::{self};

use chrono::DateTime;
use chrono::Utc;
use crate::CONFIG;
use crate::db::Database;
use crate::db::Domain;
use crate::domain::Dname;
use rand;
use rocket_contrib::json::Json;
use rocket;
use rocket::get;
use rocket::post;
use rocket::request::FromRequest;
use rocket::http::RawStr;
use rocket::request::FromParam;
use rocket::request::Outcome;
use rocket::request::Request;
use rocket::response::content;
use rocket::response::content::Html;
use rocket::response::status::{self, NotFound};
use rocket::routes;
use rocket::State;
use serde_json as json;
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::net::IpAddr;
use std::str::FromStr;
use super::AppState;


const DNS_REFRESH: usize = 10;
const DNS_RETRY: usize = 5;
const DNS_EXPIRE: usize = 86400; // 24 hours
const DNS_MINIMUM: usize = 5; // aka TTL

#[derive(Clone, Debug, Serialize)]
pub enum QType {
	A,
	AAAA,
	SOA,
}

#[derive(Clone, Debug, Serialize)]
pub struct DnsResponse {
	result: Vec<DnsRecord>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DnsRecord {
	// AAAA
	qtype: QType,
	// www.example.com
	qname: String,
	// 203.0.113.2
	content: String,
	// 60
	ttl: usize,
}


impl FromStr for QType {
	type Err = String;

	fn from_str(a: &str) -> Result<Self, Self::Err> {
		match a.to_uppercase().as_str() {
			"A" => Ok(Self::A),
			"AAAA" => Ok(Self::AAAA),
			"SOA" => Ok(Self::SOA),
			_ => Err("unsupported qtype".to_string()),
		}
	}
}

impl<'r> FromParam<'r> for QType {
	type Error = String;

	fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
		param.url_decode().unwrap().parse()
	}
}

#[get("/lookup/<domain>/SOA")]
pub fn lookup_soa(
	// state: State<AppState>,
	domain: Dname
) -> Result<Json<DnsResponse>, NotFound<()>> {
	info!("SOA {:?}", domain);

	let domain_config = match CONFIG.domain.iter().find(|d| domain.ends_with(&Dname::new(d.name.clone()))) {
		Some(r) => r,
		None => panic!("not found"),
	};


	info!("using config: {:#?}", domain_config);

	// dns1.icann.org. hostmaster.icann.org. 2012081600 7200 3600 1209600 3600
	let res = format!(
		"{}. {}. {} {} {} {} {}",
		CONFIG.dns.master, CONFIG.dns.rname, 42, DNS_REFRESH, DNS_RETRY, DNS_EXPIRE, DNS_MINIMUM
	);

	let res = DnsResponse {
		result: vec![DnsRecord {
			qtype: QType::SOA,
			qname: "ffhl.de.".to_string(),
			content: res,
			ttl: DNS_MINIMUM,
		}],
	};

	info!("{:#?}", res);
	Ok(Json(res))
}

#[get("/lookup/<domain>/A")]
pub fn lookup_a(state: State<AppState>, domain: String) -> Result<Json<DnsResponse>, NotFound<()>> {
	info!("A {:?}", domain);
	let db = &state.db;

	let d = match db.get_domain(&domain.to_string()) {
		Some(r) => r,
		None => {
			info!("{:#?} was not found", domain);
			return Err(NotFound(()));
		}
	};

	let mut res = DnsResponse {
		result: vec![],
	};

	if let Some(ip) = d.ipv4 {
		res.result.push(DnsRecord {
			qtype: QType::A,
			qname: domain,
			content: ip.to_string(),
			ttl: DNS_MINIMUM,
		})
	}


	info!("{:#?}", res);
	Ok(Json(res))
}

#[get("/lookup/<domain>/AAAA")]
pub fn lookup_aaaa(state: State<AppState>, domain: String) -> Result<Json<DnsResponse>, NotFound<()>> {
	info!("AAAA {:?}", domain);
	let db = &state.db;

	let d = match db.get_domain(&domain.to_string()) {
		Some(r) => r,
		None => {
			info!("{:#?} was not found", domain);
			return Err(NotFound(()));
		}
	};

	let mut res = DnsResponse {
		result: vec![],
	};

	if let Some(ip) = d.ipv6 {
		res.result.push(DnsRecord {
			qtype: QType::AAAA,
			qname: domain,
			content: ip.to_string(),
			ttl: DNS_MINIMUM,
		})
	}

	Ok(Json(res))
}

#[get("/lookup/<domain>/ANY")]
pub fn lookup_any(
	state: State<AppState>,
	domain: Dname,
) -> Result<Json<DnsResponse>, NotFound<()>> {
	info!("ANY {}", domain);
	let db = &state.db;

	let d = match db.get_domain(&domain.to_string()) {
		Some(r) => r,
		None => {
			info!("{:#?} was not found", domain);
			return Err(NotFound(()));
		}
	};

	info!("{:#?}", d);

	let mut res = DnsResponse {
		result: vec![],
	};

	if let Some(record) = d.ipv4 {
		res.result.push(DnsRecord {
			qtype: QType::A,
			qname: domain.to_string(),
			content: record.to_string(),
			ttl: DNS_MINIMUM,
		});
	}

	if let Some(record) = d.ipv6 {
		res.result.push(DnsRecord {
			qtype: QType::AAAA,
			qname: domain.to_string(),
			content: record.to_string(),
			ttl: DNS_MINIMUM,
		});
	}

	info!("{:#?}", res);
	Ok(Json(res))
}

#[get("/getAllDomainMetadata/<domain>")]
pub fn lookup_getalldomainmetadata(state: State<AppState>, domain: String) -> Json<json::Value> {
	let res = json!({
		"result":{
			"PRESIGNED":["0"]
		}
	});

	Json(res)
}
