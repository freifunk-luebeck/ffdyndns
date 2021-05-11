use log::{error, info, debug};
use tera::{self};

use crate::db::Database;
use crate::db::Domain;
use chrono::DateTime;
use chrono::Utc;
use rand;
use rocket;
use rocket::get;
use rocket::post;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::request::Request;
use rocket::response::content;
use rocket::response::content::Html;
use rocket::routes;
use rocket::State;
use serde_json as json;
use serde_json::json;
use serde::{Serialize, Deserialize};
use std::fmt::{self, Display};
use std::net::IpAddr;
use super::AppState;
use rocket_contrib::json::Json;

#[derive(Clone, Debug, Serialize)]
pub enum QType {
	A,
	AAAA,
	SOA,
}

#[derive(Clone, Debug, Serialize)]
pub struct DnsResponse {
	result: Vec<DnsRecord>
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
	ttl: usize
}

#[get("/lookup/<domain>/<record>")]
pub fn lookup(state: State<AppState>, domain: String, record: String) -> Json<DnsResponse> {
	info!("{:?} {:?}", record, domain);
	let res = DnsResponse {
		result: vec![
			DnsRecord {
				qtype: QType::A,
				qname: "ffhl.de.".to_string(),
				content: "1.1.1.1".to_string(),
				ttl: 60
			}
		]
	};

	info!("{:#?}", res);

	Json(res)
}
